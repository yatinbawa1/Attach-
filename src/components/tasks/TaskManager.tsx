import {useEffect, useState} from 'react';
import {Box, Button, Flex, HStack, IconButton, Input, Text, Textarea, VStack} from '@chakra-ui/react';
import {FaLink, FaParagraph, FaPlay, FaPlus, FaTrash} from 'react-icons/fa';

// Internal Imports
import {useStore} from '@/store.ts';
import {SocialMedia} from '@/types.ts';
import {getPlatformConfig} from '@/utils/platformConfig.ts';
import {invoke} from "@tauri-apps/api/core";

export const TaskManager = () => {
    
    // Local State
    const [link, setLink] = useState('');
    const [comment, setComment] = useState('');
    const [isValidLink, setIsValidLink] = useState(true);

    // Store State/Actions
    const {tasks, addTask, removeTask, setError} = useStore();
    


    /**
     * Real-time Regex Validation: Link must start with https://
     */
    useEffect(() => {
        if (link === "") {
            setIsValidLink(true);
            return;
        }
        const urlPattern = /^https:\/\/.+/;
        setIsValidLink(urlPattern.test(link.toLowerCase()));
    }, [link]);

    /**
     * Handles adding a task to the scheduler
     */
    const handleAddToScheduler = () => {
        // 1. Basic Validation
        if (!link.trim() || !isValidLink) {
            setError("Please provide a valid URL starting with https://");
            return;
        }

        // 2. Platform Detection
        let platform: SocialMedia | null = null;
        const lowerLink = link.toLowerCase();

        if (lowerLink.includes('youtube.com') || lowerLink.includes('youtu.be')) {
            platform = SocialMedia.Youtube;
        } else if (lowerLink.includes('facebook.com')) {
            platform = SocialMedia.Facebook;
        } else if (lowerLink.includes('instagram.com')) {
            platform = SocialMedia.Instagram;
        } else if (lowerLink.includes('twitter.com') || lowerLink.includes('x.com')) {
            platform = SocialMedia.X;
        }

        if (!platform) {
            setError("Supported platform (YouTube, FB, IG, X) not detected in link.");
            return;
        }

        // 3. Add to Store
        addTask(link, comment, platform);

        // 4. Reset Inputs
        setLink('');
        setComment('');
    };

    const handleStartAutomation = async () => {
        if (tasks.length === 0) {
            setError("No tasks to execute");
            return;
        }

        const hasBriefcases = tasks.some(task => task.related_brief_cases && task.related_brief_cases.length > 0);
        if (!hasBriefcases) {
            setError("No profiles available for any tasks. Please add profiles and briefcases first.");
            return;
        }

        try {
            const result = await invoke("start_automation", {
                tasksJson: JSON.stringify(tasks),
            });
            console.log('Automation started:', result);
        } catch (error) {
            console.error('Error in handleStartAutomation:', error);
            setError(`Failed to start automation: ${error}`);
        }
    }

    return (
        <VStack gap={6} align="stretch" h="100%" overflow="hidden">

            {/* --- INPUT SECTION --- */}
            <Box
                p={6}
                borderRadius="2xl"
                bg="whiteAlpha.50"
                backdropFilter="blur(20px)"
                border="1px solid"
                borderColor="whiteAlpha.100"
                boxShadow="0 8px 32px 0 rgba(0, 0, 0, 0.37)"
            >
                <VStack gap={5} align="stretch">
                    <Box>
                        <HStack mb={2} color="gray.400">
                            <FaLink size={12}/>
                            <Text fontSize="xs" fontWeight="bold" textTransform="uppercase" letterSpacing="widest">
                                Target Link
                            </Text>
                        </HStack>
                        <Input
                            placeholder="https://www.youtube.com/watch?v=..."
                            value={link}
                            onChange={(e) => setLink(e.target.value)}
                            bg="blackAlpha.400"
                            variant="subtle"
                            borderColor={isValidLink ? "whiteAlpha.200" : "red.500"}
                            _focus={{
                                borderColor: isValidLink ? "blue.500" : "red.500",
                                bg: "blackAlpha.600"
                            }}
                            transition="all 0.2s"
                        />
                        {!isValidLink && (
                            <Text color="red.400" fontSize="2xs" mt={1} fontWeight="medium">
                                Link must be a valid HTTPS URL
                            </Text>
                        )}
                    </Box>

                    <Box>
                        <HStack mb={2} color="gray.400">
                            <FaParagraph size={12}/>
                            <Text fontSize="xs" fontWeight="bold" textTransform="uppercase" letterSpacing="widest">
                                Automation Context / Instructions
                            </Text>
                        </HStack>
                        <Textarea
                            placeholder="Enter comments, instructions, or hashtags..."
                            value={comment}
                            onChange={(e) => setComment(e.target.value)}
                            bg="blackAlpha.400"
                            variant="subtle"
                            rows={3}
                            _focus={{borderColor: "blue.500", bg: "blackAlpha.600"}}
                        />
                    </Box>

                    <Flex justify="flex-end">
                        <Button
                            colorPalette="blue"
                            onClick={handleAddToScheduler}
                            disabled={!isValidLink || link === ""}
                            _hover={{transform: "translateY(-1px)", boxShadow: "0 0 15px rgba(66, 153, 225, 0.4)"}}
                            transition="all 0.2s"
                        >
                            <FaPlus style={{marginRight: '8px'}}/>
                            Add to Scheduler
                        </Button>
                    </Flex>
                </VStack>
            </Box>

            {/* --- PENDING TASKS SECTION --- */}
            <VStack align="stretch" flex={1} overflow="hidden">
                <HStack justify="space-between" px={1}>
                    <HStack gap={3}>
                        <Text fontSize="lg" fontWeight="extrabold">Pending Tasks</Text>
                        <Box px={2} py={0.5} bg="blue.500" borderRadius="full">
                            <Text fontSize="xs" fontWeight="bold">{tasks.length}</Text>
                        </Box>
                    </HStack>
                </HStack>

                {/* Internal Scrollable Task Area */}
                <Box
                    flex={1}
                    overflowY="auto"
                    pr={2}
                    css={{
                        '&::-webkit-scrollbar': {width: '4px'},
                        '&::-webkit-scrollbar-track': {background: 'transparent'},
                        '&::-webkit-scrollbar-thumb': {background: 'rgba(255,255,255,0.1)', borderRadius: '10px'},
                    }}
                >
                    {tasks.length === 0 ? (
                        <Flex direction="column" align="center" justify="center" h="100%" color="whiteAlpha.300">
                            <FaLink size={30} style={{marginBottom: '12px'}}/>
                            <Text fontSize="sm">No tasks currently scheduled</Text>
                        </Flex>
                    ) : (
                        tasks.map(task => {
                            const config = getPlatformConfig(task.social_media);
                            return (
                                <Box
                                    key={task.task_id}
                                    p={4}
                                    mb={3}
                                    bg="whiteAlpha.50"
                                    borderRadius="xl"
                                    border="1px solid"
                                    borderColor="whiteAlpha.100"
                                    _hover={{borderColor: config.color, bg: "whiteAlpha.100"}}
                                    transition="all 0.2s"
                                >
                                    <Flex justify="space-between" align="center">
                                        <HStack gap={4}>
                                            <Flex
                                                p={2.5}
                                                bg={config.bg}
                                                color={config.color}
                                                borderRadius="lg"
                                                boxShadow={`0 0 10px ${config.color}30`}
                                            >
                                                <config.icon size={18}/>
                                            </Flex>
                                            <Box maxW="400px">
                                                <Text fontSize="sm" fontWeight="bold" color="white" lineClamp={1}>
                                                    {task.link}
                                                </Text>
                                                <Text fontSize="xs" color="gray.500" lineClamp={1}>
                                                    {task.comment_unformatted || "No additional instructions provided."}
                                                </Text>
                                            </Box>
                                        </HStack>

                                        <IconButton
                                            aria-label="Remove Task"
                                            size="sm"
                                            variant="ghost"
                                            colorPalette="red"
                                            onClick={() => removeTask(task.task_id)}
                                            _hover={{bg: "red.900/20"}}
                                        >
                                            <FaTrash/>
                                        </IconButton>
                                    </Flex>
                                </Box>
                            )
                        })
                    )}
                </Box>

                {/* Global Action Button */}
                <Box pt={4}>
                    <Button
                        size="lg"
                        colorPalette="blue"
                        w="full"
                        h="60px"
                        onClick={handleStartAutomation}
                        boxShadow="0 4px 20px rgba(0, 0, 0, 0.4)"
                    >
                        <FaPlay style={{marginRight: '10px'}}/>
                        Start Automation Sequence
                    </Button>
                </Box>
            </VStack>
        </VStack>
    );
};