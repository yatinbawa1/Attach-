import {Box, Button, Flex, HStack, IconButton, Menu, Text} from '@chakra-ui/react';
import {FaCheckCircle, FaEllipsisV, FaSignInAlt, FaTrash} from 'react-icons/fa';
import {useStore} from '@/store.ts';
import {BriefCase, SocialMedia} from '@/types.ts';
import {getPlatformConfig} from '@/utils/platformConfig.ts';
import {openLoginWindow} from "@/api/tauriCommands.ts";

export const BriefcaseItem = ({briefcase}: { briefcase: BriefCase }) => {
    const {removeBriefcase, toggleBriefcaseActive, profiles} = useStore();
    const config = getPlatformConfig(briefcase.social_media);

    const handleLoginClick = async () => {
        let link: String = "";

        if (briefcase.social_media == SocialMedia.Facebook) {
            link = "https://facebook.com/";
        } else if (briefcase.social_media == SocialMedia.Instagram) {
            link = "https://instagram.com/";
        } else if (briefcase.social_media == SocialMedia.Youtube) {
            link = "https://www.youtube.com/";
        } else if (briefcase.social_media == SocialMedia.X) {
            link = "https://www.x.com/";
        } else {
            link = "https://example.com/"
        }

        profiles.map(async (profile) => {
            if (profile.profile_id === briefcase.profile_id) {
                await openLoginWindow(profile, link);
            }
        })
    }

    return (
        <Flex
            align="center"
            justify="space-between"
            p={3}
            mb={2}
            borderRadius="xl"
            position="relative"
            overflow="hidden"
            bg={config.bg}
            backdropFilter="blur(12px)"
            border="1px solid"
            borderColor={config.borderColor}
            transition="all 0.3s cubic-bezier(0.4, 0, 0.2, 1)"
            _hover={{
                transform: "translateX(4px)",
                borderColor: config.color,
                boxShadow: `0 0 15px ${config.bg}`
            }}
        >
            <HStack gap={3}>
                {/* Icon with Platform Tint */}
                <Flex
                    w={10} h={10}
                    align="center" justify="center"
                    borderRadius="lg"
                    bg="blackAlpha.400"
                    color={config.color}
                >
                    <config.icon size={20}/>
                </Flex>
                <Box>
                    <Text fontSize="xs" color="whiteAlpha.600">{briefcase.social_media}</Text>
                </Box>
                <Button
                    size="sm"
                    variant="ghost"
                    color="whiteAlpha.800"
                    bg="whiteAlpha.100"
                    borderColor="whiteAlpha.300"
                    borderWidth="1px"
                    _hover={{
                        bg: `${config.color}15`,
                        borderColor: config.color,
                        color: config.color,
                        transform: "scale(1.05)",
                        boxShadow: `0 0 10px ${config.color}40`
                    }}
                    _active={{
                        bg: `${config.color}25`,
                        transform: "scale(0.98)"
                    }}
                    onClick={async (_) => {
                        await handleLoginClick();
                    }}
                >
                    <FaSignInAlt style={{marginRight: '6px'}} size={12}/>
                    Open
                </Button>
            </HStack>

            {/* Menu Actions */}
            <Menu.Root>
                <Menu.Trigger asChild>
                    <IconButton
                        variant="ghost"
                        size="sm"
                        color="whiteAlpha.700"
                        _hover={{bg: "whiteAlpha.200", color: "white"}}
                    >
                        <FaEllipsisV/>
                    </IconButton>
                </Menu.Trigger>
                <Menu.Content bg="gray.900" borderColor="whiteAlpha.200">
                    <Menu.Item value="login" onSelect={() => {/* TODO: invoke tauri login */
                    }}>
                        <FaSignInAlt style={{marginRight: '8px'}}/> Login
                    </Menu.Item>
                    <Menu.Item value="mark" onSelect={() => toggleBriefcaseActive(briefcase.id)}>
                        <FaCheckCircle style={{marginRight: '8px'}}/> Mark Logged In
                    </Menu.Item>
                    <Menu.Item value="delete" color="red.400" onSelect={() => removeBriefcase(briefcase.id)}>
                        <FaTrash style={{marginRight: '8px'}}/> Delete
                    </Menu.Item>
                </Menu.Content>
            </Menu.Root>
        </Flex>
    );
};