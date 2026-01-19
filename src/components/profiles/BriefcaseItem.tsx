import {Box, Flex, HStack, IconButton, Menu, Text} from '@chakra-ui/react';
import {FaCheckCircle, FaEllipsisV, FaSignInAlt, FaTrash} from 'react-icons/fa';
import {useStore} from '../../store';
import {BriefCase} from '../../types';
import {getPlatformConfig} from '../../utils/platformConfig';

export const BriefcaseItem = ({briefcase}: { briefcase: BriefCase }) => {
    const {removeBriefcase, toggleBriefcaseActive} = useStore();
    const config = getPlatformConfig(briefcase.platform);

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
                    <HStack gap={2}>
                        <Text fontWeight="bold" fontSize="sm" color="white">{briefcase.user_name}</Text>
                        {/* Status Indicator */}
                        <Box
                            w={2} h={2}
                            borderRadius="full"
                            bg={briefcase.is_active ? "green.400" : "red.500"}
                            boxShadow={briefcase.is_active ? "0 0 10px #48BB78" : "none"}
                        />
                    </HStack>
                    <Text fontSize="xs" color="whiteAlpha.600">{briefcase.platform}</Text>
                </Box>
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
                    <Menu.Item value="login" onClick={() => {/* TODO: invoke tauri login */
                    }}>
                        <FaSignInAlt style={{marginRight: '8px'}}/> Login
                    </Menu.Item>
                    <Menu.Item value="mark" onClick={() => toggleBriefcaseActive(briefcase.id)}>
                        <FaCheckCircle style={{marginRight: '8px'}}/> Mark Logged In
                    </Menu.Item>
                    <Menu.Item value="delete" color="red.400" onClick={() => removeBriefcase(briefcase.id)}>
                        <FaTrash style={{marginRight: '8px'}}/> Delete
                    </Menu.Item>
                </Menu.Content>
            </Menu.Root>
        </Flex>
    );
};