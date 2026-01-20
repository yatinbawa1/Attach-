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
    Spinner,
    Text,
    VStack
} from '@chakra-ui/react';
import {FaArrowRight} from 'react-icons/fa';

import {closeWorkspace, getPanelData, nextWorkspaceItem, copyToClipboard, setCommentIndex} from '../../api/tauriCommands';
import {BriefCase, Profile, Task} from '@/types';

export const Panel = () => {
    const [currentTask, setCurrentTask] = useState<Task | null>(null);
    const [currentProfile, setCurrentProfile] = useState<Profile | null>(null);
    const [briefcases, setBriefcases] = useState<BriefCase[]>([]);
    const [visitedBriefcases, setVisitedBriefcases] = useState(0);
    const [totalBriefcases, setTotalBriefcases] = useState(0);
    const [isComplete, setIsComplete] = useState(false);
    const [isLoading, setIsLoading] = useState(false);
    const [currentComment, setCurrentComment] = useState<string | null>(null);
    const [copiedText, setCopiedText] = useState<string | null>(null);

    const [taskProgress, setTaskProgress] = useState<[number, number] | null>(null);

    useEffect(() => {
        if (currentComment && copiedText !== currentComment) {
            copyToClipboard(currentComment).catch(err => {
                console.error('Failed to copy comment to clipboard:', err);
            });
            setCopiedText(currentComment);
        }
    }, [currentComment]);

    useEffect(() => {
        const loadData = async () => {
            try {
                const data = await getPanelData() as any;
                if (data && typeof data === 'object') {
                    const previousTaskId = currentTask?.task_id;
                    const previousProfileId = currentProfile?.profile_id;

                    setCurrentTask(data.current_task || null);
                    setCurrentProfile(data.current_profile || null);
                    setBriefcases(data.briefcases || []);
                    const overallProgress = data.overall_progress || [0, 0];
                    setVisitedBriefcases(overallProgress[0] || 0);
                    setTotalBriefcases(overallProgress[1] || 0);
                    setIsComplete(overallProgress[0] >= overallProgress[1] && overallProgress[1] > 0);

                    const currentIdx = data.current_task_index;
                    const taskProgressList = data.task_progress || [];
                    const currentTaskProgress = currentIdx !== null ? taskProgressList[currentIdx] : null;
                    setTaskProgress(currentTaskProgress ? [currentTaskProgress[1], currentTaskProgress[2]] : null);

                    const newComment = data.current_comment || null;
                    setCurrentComment(newComment);

                    const currentTaskId = data.current_task?.task_id;
                    const currentProfileId = data.current_profile?.profile_id;

                    if (previousTaskId !== currentTaskId || previousProfileId !== currentProfileId) {
                        setIsLoading(false);
                    }

                    const newTask = data.current_task;
                    if (newTask && newComment && newTask.comments[newTask.comment_index] === newComment) {
                        setCopiedText(newComment);
                    }
                }
            } catch (error) {
                console.error('Failed to load panel data:', error);
            }
        };

        loadData();

        const interval = setInterval(loadData, 1000);
        return () => clearInterval(interval);
    }, [currentTask, currentProfile]);

    const currentTaskBriefcaseCount = taskProgress ? taskProgress[1] : (currentTask?.related_brief_cases?.length || 1);
    const currentTaskBriefcasesVisited = taskProgress ? taskProgress[0] : 0;
    const currentTaskBriefcasesDisplay = Math.min(currentTaskBriefcasesVisited + 1, currentTaskBriefcaseCount);
    const currentTaskProgress = currentTaskBriefcaseCount > 0 ? (currentTaskBriefcasesVisited / currentTaskBriefcaseCount) * 100 : 0;
    const overallProgress = totalBriefcases > 0 ? (visitedBriefcases / totalBriefcases) * 100 : 0;

    const handleQuit = useCallback(async () => {
        try {
            await closeWorkspace();
        } catch (error) {
            console.error('Failed to quit:', error);
        }
    }, []);

    const handleNext = useCallback(async () => {
        setIsLoading(true);
        const timeout = setTimeout(() => setIsLoading(false), 3000);
        try {
            await nextWorkspaceItem();
        } catch (error) {
            console.error('Failed to go to next:', error);
            setIsLoading(false);
            clearTimeout(timeout);
        }
    }, []);

    const handleCommentSelect = useCallback(async (commentIndex: number) => {
        if (currentTask) {
            try {
                await setCommentIndex(currentTask.comment_index, commentIndex);
            } catch (error) {
                console.error('Failed to set comment index:', error);
            }
        }
    }, [currentTask]);

    useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            if (e.key === "ArrowRight" && !isLoading) handleNext();
            if (e.key === "Escape") handleQuit();
        };

        window.addEventListener('keydown', handleKeyDown);
        return () => window.removeEventListener('keydown', handleKeyDown);
    }, [handleQuit, handleNext, isLoading]);

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
                                        {currentTask.related_brief_cases && currentTask.related_brief_cases.length > 0 ? `${currentTaskBriefcasesDisplay}/${currentTaskBriefcaseCount}` : 'No briefcases'}
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
                {currentTask && currentTask.comments.length > 0 && (
                    <CardRoot bg="whiteAlpha.100" backdropFilter="blur(10px)" border="1px solid"
                              borderColor="whiteAlpha.200">
                        <CardBody>
                            <VStack align="start" gap={3}>
                                <HStack justify="space-between">
                                    <Heading size="sm">Comments</Heading>
                                    <Text fontSize="xs" color="green.400">
                                        {copiedText === currentComment ? 'Copied to clipboard' : ''}
                                    </Text>
                                </HStack>
                                <VStack align="start" gap={2} w="100%">
                                    {currentTask.comments.map((comment, index) => (
                                        <Box
                                            key={index}
                                            p={3}
                                            borderRadius="md"
                                            cursor="pointer"
                                            transition="all 0.2s"
                                            bg={index === currentTask.comment_index ? "blue.900" : "whiteAlpha.50"}
                                            border="1px solid"
                                            borderColor={index === currentTask.comment_index ? "blue.500" : "transparent"}
                                            _hover={{
                                                bg: index === currentTask.comment_index ? "blue.800" : "whiteAlpha.100",
                                                borderColor: index === currentTask.comment_index ? "blue.400" : "blue.500"
                                            }}
                                            onClick={() => handleCommentSelect(index)}
                                            w="100%"
                                        >
                                            <Text
                                                fontSize="sm"
                                                color={index === currentTask.comment_index ? "white" : "whiteAlpha.900"}
                                            >
                                                {index + 1}. {comment}
                                            </Text>
                                        </Box>
                                    ))}
                                </VStack>
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
                                    {/* Previous button disabled - backend functionality not implemented */}
                                    {/* <Button
                                        flex={1}
                                        variant="ghost"
                                        onClick={() => prevWorkspaceItem().catch(console.error)}
                                    >
                                        <FaArrowLeft style={{marginRight: '8px'}}/>
                                        Previous
                                    </Button> */}
                                    <Button
                                        flex={1}
                                        variant="ghost"
                                        onClick={handleNext}
                                        disabled={isLoading}
                                    >
                                        {isLoading ? (
                                            <Spinner size="sm" />
                                        ) : (
                                            <>
                                                Next <FaArrowRight style={{marginLeft: '8px'}}/>
                                            </>
                                        )}
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