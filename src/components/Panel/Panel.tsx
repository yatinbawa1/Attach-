import {useCallback, useEffect, useState} from 'react';
import {
    Box,
    Button,
    CardBody,
    CardRoot,
    Flex,
    Heading,
    HStack,
    ProgressRoot,
    ProgressTrack,
    Text,
    VStack
} from '@chakra-ui/react';
import {FaArrowLeft} from 'react-icons/fa';
import {closeWorkspace, getPanelData, nextWorkspaceItem, prevWorkspaceItem,} from '../../api/tauriCommands';
import {BriefCase, Profile, Task} from '@/types';

export const Panel = () => {
    const [currentTask, setCurrentTask] = useState<Task | null>(null);
    const [currentProfile, setCurrentProfile] = useState<Profile | null>(null);
    const [briefcases, setBriefcases] = useState<BriefCase[]>([]);
    const [visitedBriefcases, setVisitedBriefcases] = useState(0);
    const [totalBriefcases, setTotalBriefcases] = useState(0);
    const [isComplete, setIsComplete] = useState(false);

    // Load data from Tauri state on mount
    useEffect(() => {
        const loadData = async () => {
            try {
                const data = await getPanelData() as any;
                if (data && typeof data === 'object') {
                    setCurrentTask(data.current_task || null);
                    setCurrentProfile(data.current_profile || null);
                    setBriefcases(data.briefcases || []);
                    setVisitedBriefcases(data.visited_briefcases || 0);
                    setTotalBriefcases(data.total_briefcases || 0);
                    setIsComplete(data.visited_briefcases >= data.total_briefcases && data.total_briefcases > 0);
                }
            } catch (error) {
                console.error('Failed to load panel data:', error);
            }
        };

        loadData();

        // Poll for updates every second
        const interval = setInterval(loadData, 1000);
        return () => clearInterval(interval);
    }, []);

    // Calculate progress
    const currentTaskBriefcaseCount = currentTask?.related_brief_cases?.length || 1;
    const currentTaskBriefcasesVisited = currentTask?.progress || 0;
    const currentTaskProgress = currentTask ? ((currentTaskBriefcasesVisited) / currentTaskBriefcaseCount) * 100 : 0;
    const overallProgress = totalBriefcases > 0 ? (visitedBriefcases / totalBriefcases) * 100 : 0;

    const handleQuit = useCallback(async () => {
        try {
            await closeWorkspace(currentProfile?.profile_name);
        } catch (error) {
            console.error('Failed to quit:', error);
        }
    }, []);

    // Keyboard shortcuts
    useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            if (e.key === "ArrowLeft") prevWorkspaceItem().catch(console.error);
            if (e.key === "ArrowRight") nextWorkspaceItem();
            if (e.key === "Escape") handleQuit();
        };

        window.addEventListener('keydown', handleKeyDown);
        return () => window.removeEventListener('keydown', handleKeyDown);
    }, [handleQuit]);

    if (!currentTask) {
        return (
            <Box p={6} bg="#0a0c14" color="white" h="100vh">
                <Flex direction="column" align="center" justify="center" h="100%" gap={6}>
                    <Heading size="lg" color="#718096">No task data available</Heading>
                    <Text color="#4a5568" textAlign="center">Waiting for automation to start</Text>
                    <Text color="#4a5568" textAlign="center">Briefcases: {briefcases.length}</Text>
                </Flex>
            </Box>
        );
    }

    return (
        <Box p={6} bg="#0a0c14" color="white" h="100vh" overflow="auto">
            <VStack gap={6} align="stretch" maxW="100%">
                {/* Header */}
                <CardRoot bg="whiteAlpha.100" backdropFilter="blur(10px)" border="1px solid"
                          borderColor="whiteAlpha.200">
                    <CardBody>
                        <VStack align="start" gap={2}>
                            <Heading size="md">
                                {currentProfile?.profile_name || 'Unknown Profile'}
                            </Heading>
                            <HStack justify="space-between" w="100%">
                                <Text fontSize="sm" color="whiteAlpha.700">
                                    Platform: {currentTask?.social_media}
                                </Text>
                                <Text fontSize="sm" color="whiteAlpha.700">
                                    Briefcases: {visitedBriefcases}/{totalBriefcases}
                                </Text>
                            </HStack>
                            <Text fontSize="xs" color="whiteAlpha.600" truncate>
                                {currentTask?.link}
                            </Text>
                        </VStack>
                    </CardBody>
                </CardRoot>

                {/* Progress Section */}
                <CardRoot bg="whiteAlpha.100" backdropFilter="blur(10px)" border="1px solid"
                          borderColor="whiteAlpha.200">
                    <CardBody>
                        <VStack align="stretch" gap={4}>
                            <Heading size="sm">Progress</Heading>

                            {/* Current Task Progress */}
                            <Box>
                                <HStack justify="space-between" mb={2}>
                                    <Text fontSize="xs" color="whiteAlpha.700">Current Task</Text>
                                    <Text fontSize="xs" color="whiteAlpha.700">
                                        {currentTask && currentTask.related_brief_cases ? `${currentTask.progress}/${currentTask.related_brief_cases.length}` : 'No briefcases'}
                                    </Text>
                                </HStack>
                                <ProgressRoot value={currentTaskProgress} colorPalette="blue">
                                    <ProgressTrack/>
                                </ProgressRoot>
                            </Box>

                            {/* Overall Progress */}
                            <Box>
                                <HStack justify="space-between" mb={2}>
                                    <Text fontSize="xs" color="whiteAlpha.700">Overall Progress</Text>
                                    <Text fontSize="xs" color="whiteAlpha.700">
                                        {visitedBriefcases}/{totalBriefcases}
                                    </Text>
                                </HStack>
                                <ProgressRoot value={overallProgress} colorPalette="green">
                                    <ProgressTrack/>
                                </ProgressRoot>
                            </Box>

                            {isComplete && (
                                <Box bg="green.900" p={3} borderRadius="md" textAlign="center">
                                    <Text color="green.200" fontWeight="bold">All Tasks Complete!</Text>
                                    <Text fontSize="xs" color="green.300">Press Quit to exit</Text>
                                </Box>
                            )}
                        </VStack>
                    </CardBody>
                </CardRoot>

                {/* Current Comment */}
                {currentTask.comments && currentTask.comments.length > 0 && (
                    <CardRoot bg="whiteAlpha.100" backdropFilter="blur(10px)" border="1px solid"
                              borderColor="whiteAlpha.200">
                        <CardBody>
                            <VStack align="start" gap={3}>
                                <Heading size="sm">Current Comment</Heading>
                                <Text
                                    fontSize="sm"
                                    bg="whiteAlpha.50"
                                    p={3}
                                    borderRadius="md"
                                    w="100%"
                                >
                                    {currentTask.comments[currentTask.progress] || 'No comment available'}
                                </Text>
                            </VStack>
                        </CardBody>
                    </CardRoot>
                )}

                {/* Controls */}
                <CardRoot bg="whiteAlpha.100" backdropFilter="blur(10px)" border="1px solid"
                          borderColor="whiteAlpha.200">
                    <CardBody>
                        <VStack gap={3}>
                            {/* Navigation */}
                            <HStack w="100%" gap={3}>
                                <Button
                                    flex={1}
                                    variant="ghost"
                                    onClick={() => prevWorkspaceItem().catch(console.error)}
                                >
                                    <FaArrowLeft style={{marginRight: '8px'}}/>
                                    Previous
                                </Button>
                                <Button
                                    flex={1}
                                    variant="ghost"
                                    onClick={() => {
                                        nextWorkspaceItem().catch(console.error);
                                    }}
                                >
                                    Next
                                </Button>
                            </HStack>

                            {/* Quit */}
                            <Button
                                w="full"
                                variant="outline"
                                colorPalette="red"
                                onClick={handleQuit}
                            >
                                Quit
                            </Button>
                        </VStack>
                    </CardBody>
                </CardRoot>

                {/* Help */}
                <Box mt="auto">
                    <Text fontSize="xs" color="whiteAlpha.500" textAlign="center">
                        Shortcuts: ← Previous | → Next | ESC Quit
                    </Text>
                </Box>
            </VStack>
        </Box>
    );
};