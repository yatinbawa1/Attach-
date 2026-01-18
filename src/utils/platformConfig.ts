import {FaFacebook, FaInstagram, FaTwitter, FaYoutube} from 'react-icons/fa';
import {SocialMedia} from '../types';

export const getPlatformConfig = (platform: SocialMedia) => {
    switch (platform) {
        case SocialMedia.Youtube:
            return {
                color: '#FF0000',
                bg: 'rgba(255, 0, 0, 0.15)',
                borderColor: 'rgba(255, 0, 0, 0.3)',
                icon: FaYoutube
            };
        case SocialMedia.Facebook:
            return {
                color: '#1877F2',
                bg: 'rgba(24, 119, 242, 0.15)',
                borderColor: 'rgba(24, 119, 242, 0.3)',
                icon: FaFacebook
            };
        case SocialMedia.Instagram:
            return {
                color: '#E4405F',
                bg: 'rgba(228, 64, 95, 0.15)',
                borderColor: 'rgba(228, 64, 95, 0.3)',
                icon: FaInstagram
            };
        case SocialMedia.X:
            return {
                color: '#FFFFFF',
                bg: 'rgba(255, 255, 255, 0.1)',
                borderColor: 'rgba(255, 255, 255, 0.2)',
                icon: FaTwitter
            };
        default:
            return {
                color: '#718096',
                bg: 'rgba(113, 128, 150, 0.1)',
                borderColor: 'rgba(113, 128, 150, 0.2)',
                icon: FaTwitter
            };
    }
};