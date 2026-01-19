import {invoke} from '@tauri-apps/api/core';
import {BriefCase, Profile, Task} from '../types';


export interface ScreenshotInfo {
    id: string;
    base64_data: string;
    width: number;
    height: number;
    timestamp: string;
    profile_id: string;
    briefcase_id: string;
    annotations: Annotation[];
}

export interface Annotation {
    x: number;
    y: number;
    overlay_path: string;
}

export const createProfile = (name: string): Promise<Profile> =>
    invoke('create_profile', {profileName: name});

export const getPanelData = (): Promise<[Task | null, Profile | null, BriefCase[], number, number]> =>
    invoke('get_panel_data');


export const closeWorkspace = (profileName: string | undefined): Promise<void> => {
    const args = profileName ? { profileName } : {};
    return invoke('close_workspace', args);
};

export const loadProfiles = (): Promise<Profile[]> =>
    invoke('load_profiles');

export const loadBriefcases = (): Promise<BriefCase[]> =>
    invoke('load_briefcases');


export const saveProfiles = (profiles: Profile[]): Promise<void> =>
    invoke('save_profiles', {profiles});

export const saveBriefcases = (briefcases: BriefCase[]): Promise<void> =>
    invoke('save_briefcases', {briefcases});

export const saveAllData = (profiles: Profile[], briefcases: BriefCase[]): Promise<void> =>
    invoke('save_all_data', {profiles, briefcases});


export const prevWorkspaceItem = (): Promise<void> =>
    invoke('prev_task_execution');

export const changeWebviewUrl = (url: string): Promise<void> =>
    invoke('change_webview_url', { url });

export const nextWorkspaceItem = (): Promise<void> =>
    invoke('next_task_execution');

export const addNewItem = (
    itemType: string,
    payload: any
): Promise<void> =>
    invoke('add_item_persist', {itemType, payload});


export const tourInvokeGetLatestScreenshot = (): Promise<string | null> =>
    invoke('tour_get_latest_screenshot');

export const goToNextBriefcase = (): Promise<void> =>
    invoke('go_to_next_briefcase');
