import {Accordion, Box, Button, Card, Flex, HStack, IconButton, Menu, Text} from '@chakra-ui/react';
import {FaEllipsisV, FaPlus, FaTrash} from 'react-icons/fa';
import {useStore} from '../../store';
import {Profile,} from '../../types';
import {BriefcaseItem} from './BriefcaseItem';

export const ProfileCard = ({profile}: { profile: Profile }) => {
    const {briefcases, removeProfile, setAddBriefcaseOpen} = useStore();
    const profileBriefcases = briefcases.filter(b => b.profile_id === profile.profile_id);

    return (
        <Card.Root variant="outline" bg="whiteAlpha.50" borderColor="whiteAlpha.100" mb={4} overflow="hidden">
            <Accordion.Root collapsible defaultValue={["info"]}>
                <Accordion.Item value="info" border="none">

                    <Flex align="center" justify="space-between" p={4} borderBottom="1px solid"
                          borderColor="whiteAlpha.100" bg="whiteAlpha.50">
                        <Box flex={1}>
                            <Text fontWeight="bold" fontSize="md">{profile.profile_name}</Text>
                            <Text fontSize="xs" color="gray.400">{profileBriefcases.length} Briefcases</Text>
                        </Box>

                        <HStack>
                            <Menu.Root>
                                <Menu.Trigger asChild>
                                    <IconButton variant="ghost" size="sm" aria-label="Profile Options">
                                        <FaEllipsisV/>
                                    </IconButton>
                                </Menu.Trigger>
                                <Menu.Content bg="gray.900" borderColor="whiteAlpha.200" zIndex={1500}>
                                    <Menu.Item value="add"
                                               onClick={() => setAddBriefcaseOpen(true, profile.profile_id)}>
                                        <FaPlus/> Add Suitcase
                                    </Menu.Item>
                                    <Menu.Item value="delete" color="red.400"
                                               onClick={() => removeProfile(profile.profile_id)}>
                                        <FaTrash/> Delete Profile
                                    </Menu.Item>
                                </Menu.Content>
                            </Menu.Root>
                            <Accordion.ItemTrigger w="auto" p={1}/>
                        </HStack>
                    </Flex>

                    <Accordion.ItemContent pb={4} pt={4} px={2}>
                        {profileBriefcases.length > 0 ? (
                            profileBriefcases.map(b => <BriefcaseItem key={b.id} briefcase={b}/>)
                        ) : (
                            <Text fontSize="sm" color="gray.500" textAlign="center" py={2}>No briefcases linked.</Text>
                        )}

                        <Button
                            size="sm" w="full" mt={2} variant="outline"
                            borderStyle="dashed"
                            borderColor="whiteAlpha.300"
                            color="gray.400"
                            _hover={{
                                borderColor: "blue.400",
                                color: "blue.400",
                                bg: "blue.900/20",
                                transform: "scale(1.02)"
                            }}
                            transition="all 0.2s cubic-bezier(0.4, 0, 0.2, 1)"
                            onClick={() => setAddBriefcaseOpen(true, profile.profile_id)}
                        >
                            <FaPlus style={{marginRight: '5px'}}/> Link Account
                        </Button>
                    </Accordion.ItemContent>
                </Accordion.Item>
            </Accordion.Root>
        </Card.Root>
    );
};