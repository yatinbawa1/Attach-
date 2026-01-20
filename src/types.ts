// types.ts
export enum SocialMedia {
    Youtube = "Youtube",
    Facebook = "Facebook",
    Instagram = "Instagram",
    X = "X",
}

export interface BriefCase {
    id: string; // Uuid
    social_media: SocialMedia;
    profile_id: string; // Uuid
    user_name: string;
    is_active: boolean;
}

export interface Profile {
    profile_id: string; // Uuid
    profile_name: string;
}

export interface Task {
    task_id: string; // Uuid
    link: string;
    comments: string[];
    comment_unformatted: string;
    progress: number;
    social_media: SocialMedia;
    related_brief_cases: BriefCase[];
    comment_index: number;
}