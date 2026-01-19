import {useState} from 'react';
import {Button, Dialog, Input, Text, VStack} from '@chakra-ui/react';
import {useStore} from '../../store';

export const AddProfileModal = () => {
    const {isAddProfileOpen, setAddProfileOpen, addProfile} = useStore();
    const [name, setName] = useState('');

    const handleSave = () => {
        // Only call addProfile if name is not empty
        if (name.trim()) {
            addProfile(name);
            setName('');
            setAddProfileOpen(false); // Manually close after success
        }
    };

    return (
        <Dialog.Root
            open={isAddProfileOpen}
            onOpenChange={(details) => setAddProfileOpen(details.open)}
        >
            <Dialog.Backdrop/>
            <Dialog.Positioner>
                <Dialog.Content
                    bg="gray.900"
                    color="white"
                    border="1px solid"
                    borderColor="whiteAlpha.200"
                >
                    <Dialog.Header>
                        <Dialog.Title>Create New Profile</Dialog.Title>
                        <Dialog.CloseTrigger/>
                    </Dialog.Header>

                    <Dialog.Body>
                        <VStack gap="4" align="stretch">
                            <Text fontSize="sm" color="gray.400">
                                Enter a unique name for this profile.
                            </Text>
                            <Input
                                placeholder="Profile Name (e.g. Startup Co.)"
                                value={name}
                                onChange={(e) => setName(e.target.value)}
                                bg="blackAlpha.400"
                                borderColor="whiteAlpha.300"
                                _focus={{borderColor: "blue.500"}}
                            />
                        </VStack>
                    </Dialog.Body>

                    <Dialog.Footer>
                        <Dialog.ActionTrigger asChild>
                            <Button variant="outline" onClick={() => setAddProfileOpen(false)}>
                                Cancel
                            </Button>
                        </Dialog.ActionTrigger>
                        <Button colorPalette="blue" onClick={handleSave}>
                            Create Profile
                        </Button>
                    </Dialog.Footer>
                </Dialog.Content>
            </Dialog.Positioner>
        </Dialog.Root>
    );
};