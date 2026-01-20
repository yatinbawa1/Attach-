import {Flex, HStack, IconButton, Text} from '@chakra-ui/react';
import {FaPlus} from 'react-icons/fa';
import {useStore} from '@/store.ts';
import {SocialMedia} from '@/types.ts';
import {getPlatformConfig} from '@/utils/platformConfig.ts'; // Extract the switch case to a util file

export const MetricsHeader = () => {
    const {getBriefcaseCount, profiles, briefcases, setAddProfileOpen, setAddBriefcaseOpen} = useStore();

    const handlePlatformCheck = (platform: SocialMedia) => {
        let targetProfileIndex = -1;
        let targetProfileId: string | null = null;

        // Logic: Find first profile WITHOUT this briefcase
        profiles.some((p, index) => {
            const hasPlatform = briefcases.some(b => b.profile_id === p.profile_id && b.social_media === platform);
            if (!hasPlatform) {
                targetProfileIndex = index;
                targetProfileId = p.profile_id;
                return true; // Break loop
            }
            return false;
        });

        if (targetProfileIndex === -1) {
            // No eligible profile found -> Launch Add Profile Modal
            // NOTE: Requirement said "if -1 launch create profile modal"
            setAddProfileOpen(true);
        } else {
            // Eligible profile found -> Launch Add Briefcase Modal for that profile
            if (targetProfileId) setAddBriefcaseOpen(true, targetProfileId, platform);
        }
    };

    return (
        <Flex gap={4} mb={6} overflowX="auto" pb={2} css={{'&::-webkit-scrollbar': {display: 'none'}}}>
            {Object.values(SocialMedia).map(platform => {
                const config = getPlatformConfig(platform);
                return (
                    <HStack key={platform} bg="whiteAlpha.100" p={2} borderRadius="lg" gap={3} border="1px solid"
                            borderColor="whiteAlpha.100" minW="140px">
                        <config.icon color={config.color}/>
                        <Text fontSize="sm" fontWeight="bold">{getBriefcaseCount(platform)}</Text>
                        <IconButton aria-label="Add" size="xs" borderRadius="full"
                                    onClick={() => handlePlatformCheck(platform)}>
                            <FaPlus/>
                        </IconButton>
                    </HStack>
                )
            })}
        </Flex>
    );
};