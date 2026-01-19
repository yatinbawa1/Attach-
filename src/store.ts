import {create} from 'zustand';
import {v4 as uuidv4} from 'uuid';
import {BriefCase, Profile, SocialMedia, Task} from './types';
import {listen} from '@tauri-apps/api/event';
import {
    createProfile,
    loadBriefcases,
    loadProfiles,
    saveAllData,
    saveBriefcases,
    saveProfiles
} from './api/tauriCommands';

interface AppState {
    profiles: Profile[];
    briefcases: BriefCase[];
    tasks: Task[];

    // UI State for Modals
    isAddProfileOpen: boolean;
    isAddBriefcaseOpen: boolean;
    activeProfileIdForBriefcase: string | null; // Which profile we are adding a briefcase to
    activePlatformForBriefcase: SocialMedia | null; // Which platform triggered the add

    // Global Error State
    error: string | null;

    // Actions
    setAddProfileOpen: (open: boolean) => void;
    setAddBriefcaseOpen: (open: boolean, profileId?: string, platform?: SocialMedia) => void;
    setError: (msg: string | null) => void;

    addProfile: (name: string) => void;
    removeProfile: (id: string) => void;
    addBriefcase: (profileId: string, platform: SocialMedia, username: string) => void;
    removeBriefcase: (id: string) => void;
    toggleBriefcaseActive: (id: string) => void;
    addTask: (link: string, comment: string, platform: SocialMedia) => void;
    removeTask: (id: string) => void;

    getBriefcaseCount: (platform: SocialMedia) => number;

    // Backend integration actions
    loadData: () => Promise<void>;
    saveData: () => Promise<void>;
    syncWithBackend: () => Promise<void>;
}

// @ts-ignore
export const useStore = create<AppState>((set, get) => ({
    profiles: [],
    briefcases: [],
    tasks: [],

    isAddProfileOpen: false,
    isAddBriefcaseOpen: false,
    activeProfileIdForBriefcase: null,
    activePlatformForBriefcase: null,
    error: null,

    // Load data from backend on initialization
    loadData: async () => {
        try {
            const [profilesData, briefcasesData] = await Promise.all([
                loadProfiles(),
                loadBriefcases()
            ]);

            set({
                profiles: profilesData,
                briefcases: briefcasesData
            });
        } catch (error) {
            console.error('Failed to load data:', error);
            set({error: `Failed to load data: ${error}`});
        }
    },

    // Save all data to backend
    saveData: async () => {
        try {
            const {profiles, briefcases} = get();
            await saveAllData(profiles, briefcases);
        } catch (error) {
            console.error('Failed to save data:', error);
            set({error: `Failed to save data: ${error}`});
        }
    },

    // Sync specific data changes to backend
    syncWithBackend: async () => {
        try {
            const {profiles, briefcases} = get();
            await Promise.all([
                saveProfiles(profiles),
                saveBriefcases(briefcases)
            ]);
        } catch (error) {
            console.error('Failed to sync data:', error);
            set({error: `Failed to sync data: ${error}`});
        }
    },

    setAddProfileOpen: (open) => set({isAddProfileOpen: open}),

    setAddBriefcaseOpen: (open, profileId = undefined, platform = undefined) =>
        set({
            isAddBriefcaseOpen: open,
            activeProfileIdForBriefcase: profileId || null,
            activePlatformForBriefcase: platform || null
        }),

    setError: (msg) => set({error: msg}),

    addProfile: async (name) => {
        if (!name.trim()) {
            get().setError("Profile name cannot be empty");
            return;
        }

        try {
            // Create profile on backend - event will update UI
            const newProfile = await createProfile(name);
            set((state: any) => ({
                profiles: [...state.profiles, newProfile],
                isAddProfileOpen: false
            }));
        } catch (error) {
            get().setError('Failed to create profile: ' + error);
        }
    },

    removeProfile: async (id) => {
        set((state) => ({
            profiles: state.profiles.filter(p => p.profile_id !== id),
            briefcases: state.briefcases.filter(b => b.profile_id !== id)
        }));

        try {
            await get().syncWithBackend();
        } catch (error) {
            console.error('Failed to remove profile:', error);
        }
    },

    addBriefcase: async (profileId, social_media, username) => {
        if (!username.trim()) {
            get().setError("Username cannot be empty");
            return;
        }

        const newBriefcase = {
            id: uuidv4(),
            social_media,
            profile_id: profileId,
            user_name: username,
            is_active: false
        };

        set((state) => ({
            briefcases: [...state.briefcases, newBriefcase],
            isAddBriefcaseOpen: false
        }));

        try {
            await saveBriefcases(get().briefcases);
        } catch (error) {
            console.error('Failed to add briefcase:', error);
        }
    },

    removeBriefcase: async (id) => {
        set((state) => ({
            briefcases: state.briefcases.filter(b => b.id !== id)
        }));

        try {
            await saveBriefcases(get().briefcases);
        } catch (error) {
            console.error('Failed to remove briefcase:', error);
        }
    },

    toggleBriefcaseActive: async (id) => {
        set((state) => ({
            briefcases: state.briefcases.map(b => b.id === id ? {...b, is_active: !b.is_active} : b)
        }));

        try {
            await saveBriefcases(get().briefcases);
        } catch (error) {
            console.error('Failed to toggle briefcase:', error);
        }
    },

    addTask: (link, comment, platform) => set((state) => {
        const matchingBriefcases = state.briefcases.filter(b => b.social_media === platform);
        return {
            tasks: [...state.tasks, {
                task_id: uuidv4(),
                link,
                comment_unformatted: comment,
                comments: [comment],
                comment_counter: 1,
                progress: 0,
                social_media: platform,
                related_brief_cases: matchingBriefcases
            }]
        };
    }),

    removeTask: (id) => set((state) => ({
        tasks: state.tasks.filter(t => t.task_id !== id)
    })),

    getBriefcaseCount: (platform) => {
        return get().briefcases.filter(b => b.social_media === platform).length;
    }
}));

// Setup event listeners once when store is created
let eventListenersSetup = false;

export const setupEventListeners = () => {
    if (eventListenersSetup) return;

    // Listen for profile changes
    listen('profiles-changed', async () => {
        try {
            const profilesData = await loadProfiles();
            useStore.setState({profiles: profilesData});
        } catch (error) {
            console.error('Failed to reload profiles:', error);
        }
    });

    // Listen for briefcase changes
    listen('briefcases-changed', async () => {
        try {
            const briefcasesData = await loadBriefcases();
            useStore.setState({briefcases: briefcasesData});
        } catch (error) {
            console.error('Failed to reload briefcases:', error);
        }
    });

    eventListenersSetup = true;
};