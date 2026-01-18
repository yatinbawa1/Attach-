import {useEffect, useState} from 'react';
import {Box, Button, createListCollection, Dialog, Input, Text, VStack} from '@chakra-ui/react';
import {useStore} from '../../store';
import {SocialMedia} from '../../types';
import {
    SelectContent,
    SelectItem,
    SelectLabel,
    SelectRoot,
    SelectTrigger,
    SelectValueText
} from "@/components/ui/select" // Assuming Chakra v3 snippet structure

export const AddBriefcaseModal = () => {
    const {
        isAddBriefcaseOpen, setAddBriefcaseOpen, addBriefcase,
        activeProfileIdForBriefcase, activePlatformForBriefcase, profiles
    } = useStore();

    const [username, setUsername] = useState('');
    const [selectedPlatform, setSelectedPlatform] = useState<SocialMedia | string>(activePlatformForBriefcase || SocialMedia.Youtube);

    // Update local state when store state changes
    useEffect(() => {
        if (activePlatformForBriefcase) setSelectedPlatform(activePlatformForBriefcase);
    }, [activePlatformForBriefcase]);

    const handleSave = () => {
        if (activeProfileIdForBriefcase) {
            addBriefcase(activeProfileIdForBriefcase, selectedPlatform as SocialMedia, username);
            setUsername('');
        }
    };

    const platformCollection = createListCollection({
        items: Object.values(SocialMedia).map(p => ({label: p, value: p}))
    });

    return (
        <Dialog.Root open={isAddBriefcaseOpen} onOpenChange={(e) => setAddBriefcaseOpen(e.open)}>
            <Dialog.Backdrop/>
            <Dialog.Positioner>
                <Dialog.Content bg="gray.900" color="white" border="1px solid" borderColor="whiteAlpha.200">
                    <Dialog.Header>
                        <Dialog.Title>Add Briefcase</Dialog.Title>
                        <Dialog.CloseTrigger/>
                    </Dialog.Header>
                    <Dialog.Body>
                        <VStack gap={4} align="stretch">
                            <Text fontSize="sm" color="gray.400">
                                Adding to: <Text as="span" fontWeight="bold" color="white">
                                {profiles.find(p => p.profile_id === activeProfileIdForBriefcase)?.profile_name || 'Unknown Profile'}
                            </Text>
                            </Text>

                            <SelectRoot collection={platformCollection} value={[selectedPlatform]}
                                        onValueChange={(e) => setSelectedPlatform(e.value[0])}>
                                <SelectLabel color="gray.400">Platform</SelectLabel>
                                <SelectTrigger>
                                    <SelectValueText placeholder="Select Platform"/>
                                </SelectTrigger>
                                <SelectContent bg="gray.800">
                                    {platformCollection.items.map((item) => (
                                        <SelectItem item={item} key={item.value}>{item.label}</SelectItem>
                                    ))}
                                </SelectContent>
                            </SelectRoot>

                            <Box>
                                <Text fontSize="xs" mb={1} color="gray.400">Username / Handle</Text>
                                <Input
                                    placeholder="@username"
                                    value={username}
                                    onChange={(e) => setUsername(e.target.value)}
                                    bg="blackAlpha.400"
                                />
                            </Box>
                        </VStack>
                    </Dialog.Body>
                    <Dialog.Footer>
                        <Button variant="outline" onClick={() => setAddBriefcaseOpen(false)}>Cancel</Button>
                        <Button colorPalette="green" onClick={handleSave}>Add Briefcase</Button>
                    </Dialog.Footer>
                </Dialog.Content>
            </Dialog.Positioner>
        </Dialog.Root>
    );
};