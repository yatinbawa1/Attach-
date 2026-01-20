import {Flex, HStack, Text, Button, Badge} from '@chakra-ui/react';
import {FaUsers, FaTasks} from 'react-icons/fa';

interface TabNavigationProps {
    activeTab: 'profiles' | 'tasks';
    onTabChange: (tab: 'profiles' | 'tasks') => void;
    profilesCount?: number;
    tasksCount?: number;
}

export const TabNavigation = ({activeTab, onTabChange, profilesCount = 0, tasksCount = 0}: TabNavigationProps) => {
    const tabs = [
        {
            id: 'profiles' as const,
            label: 'Profiles',
            icon: FaUsers,
            count: profilesCount
        },
        {
            id: 'tasks' as const,
            label: 'Tasks',
            icon: FaTasks,
            count: tasksCount
        }
    ];

    return (
        <Flex
            bg="whiteAlpha.50"
            backdropFilter="blur(20px)"
            border="1px solid"
            borderColor="whiteAlpha.100"
            borderRadius="xl"
            p={1}
            mb={6}
        >
            {tabs.map((tab) => {
                const Icon = tab.icon;
                const isActive = activeTab === tab.id;
                
                return (
                    <Button
                        key={tab.id}
                        variant="ghost"
                        flex={1}
                        h="50px"
                        bg={isActive ? "blue.500/20" : "transparent"}
                        color={isActive ? "blue.300" : "gray.400"}
                        borderColor={isActive ? "blue.500" : "transparent"}
                        borderWidth="1px"
                        borderRadius="lg"
                        onClick={() => onTabChange(tab.id)}
                        transition="all 0.2s cubic-bezier(0.4, 0, 0.2, 1)"
                        _hover={{
                            bg: isActive ? "blue.500/30" : "whiteAlpha.100",
                            color: isActive ? "blue.200" : "gray.300",
                            transform: "translateY(-1px)"
                        }}
                    >
                        <HStack gap={3}>
                            <Icon size={16} />
                            <Text fontWeight={isActive ? "bold" : "medium"} fontSize="md">
                                {tab.label}
                            </Text>
                            {tab.count > 0 && (
                                <Badge
                                    bg={isActive ? "blue.600" : "whiteAlpha.200"}
                                    color={isActive ? "white" : "gray.300"}
                                    px={2}
                                    py={0.5}
                                    borderRadius="full"
                                    fontSize="xs"
                                    fontWeight="bold"
                                >
                                    {tab.count}
                                </Badge>
                            )}
                        </HStack>
                    </Button>
                );
            })}
        </Flex>
    );
};