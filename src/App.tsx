import React from 'react';
import {Box, Button, ChakraProvider, createSystem, defaultConfig, Flex, Grid, GridItem, Text} from '@chakra-ui/react';
import {ThemeProvider} from "next-themes";
import {FaPlus} from 'react-icons/fa';
import {Toaster} from "@/components/ui/toaster"

import {useStore, setupEventListeners} from './store';
import {ErrorSystem} from './components/ui/ErrorSystem';
import {MetricsHeader} from './components/profiles/MetricsHeader';
import {ProfileCard} from './components/profiles/ProfileCard';
import {TaskManager} from './components/tasks/TaskManager';
import {TabNavigation} from './components/ui/TabNavigation';
import {AddProfileModal} from './components/modals/AddProfileModal';
import {AddBriefcaseModal} from '@/components/modals/AddBriefcaseModal.tsx';

// v3 System Setup
const system = createSystem(defaultConfig, {
    theme: {
        tokens: {
            fonts: {
                heading: {value: "Inter, sans-serif"},
                body: {value: "Inter, sans-serif"},
            },
        },
    },
});

export default function App() {
    const {setAddProfileOpen, loadData, activeTab, setActiveTab, profiles, tasks} = useStore();

    // Load data from backend on component mount
    React.useEffect(() => {
        loadData().catch(error => {
            console.error('Failed to load data on app start:', error);
        });
        
        // Setup event listeners for reactive updates
        setupEventListeners();
    }, [loadData]);

    return (
        <ChakraProvider value={system}>
            <ThemeProvider attribute="class" disableTransitionOnChange>
                <ErrorSystem/>
                <Toaster/>
                <AddProfileModal/>
                <AddBriefcaseModal/>

                 <Box h="100vh" w="100vw" bg="#0a0c14" color="white" fontFamily="Inter, sans-serif" overflow="hidden"
                     position="relative">
                     {/* Background Gradient */}
                     <Box position="fixed" top="0" left="0" right="0" bottom="0"
                          bgImage="radial-gradient(circle at 50% 50%, #1e1b4b 0%, #0a0c14 70%)" zIndex="0"
                          pointerEvents="none"/>

                     <Box
                         p={6}
                         maxW="1440px"
                         mx="auto"
                         position="relative"
                         zIndex="1"
                         h="100vh" // Full height constraint
                         display="flex"
                         flexDirection="column"
                     >
                         {/* Tab Navigation */}
                         <Box flexShrink={0}>
                             <TabNavigation
                                 activeTab={activeTab}
                                 onTabChange={setActiveTab}
                                 profilesCount={profiles.length}
                                 tasksCount={tasks.length}
                             />
                         </Box>

                         {/* Tab Content */}
                         <Box flex={1} overflow="hidden">
                             {activeTab === 'profiles' ? (
                                 // Profiles Tab Content
                                 <Box display="flex" flexDirection="column" h="100%" overflow="hidden">
                                     <Flex justify="space-between" align="center" mb={4} flexShrink={0}>
                                         <Text fontSize="xl" fontWeight="bold">Active Profiles</Text>
                                         <Button size="sm" variant="ghost" colorPalette="blue"
                                                 onClick={() => setAddProfileOpen(true)}>
                                             <FaPlus style={{marginRight: '4px'}}/> Add Profile
                                         </Button>
                                     </Flex>

                                     <Box flexShrink={0}>
                                         <MetricsHeader/>
                                     </Box>

                                     {/* Scrollable Container for Cards */}
                                     <Box
                                         flex={1}
                                         overflowY="auto"
                                         pr={2}
                                         css={{
                                             '&::-webkit-scrollbar': {width: '4px'},
                                             '&::-webkit-scrollbar-track': {background: 'transparent'},
                                             '&::-webkit-scrollbar-thumb': {background: '#2d3748', borderRadius: '4px'}
                                         }}
                                     >
                                         <Grid
                                             templateColumns={{
                                                 base: "1fr",           // Mobile: 1 column
                                                 sm: "1fr",            // Small: 1 column  
                                                 md: "repeat(2, 1fr)", // Medium: 2 columns
                                                 lg: "repeat(2, 1fr)", // Large: 2 columns
                                                 xl: "repeat(3, 1fr)", // Extra Large: 3 columns
                                                 "2xl": "repeat(4, 1fr)" // 2XL: 4 columns
                                             }}
                                             gap={6}
                                             px={2}
                                             autoRows="1fr" // Ensure all rows have equal height
                                         >
                                             {profiles.map(p => (
                                                 <GridItem key={p.profile_id}>
                                                     <ProfileCard profile={p}/>
                                                 </GridItem>
                                             ))}
                                         </Grid>
                                     </Box>
                                 </Box>
                             ) : (
                                 // Tasks Tab Content
                                 <Box h="100%" display="flex" flexDirection="column" overflow="hidden">
                                     <Text fontSize="xl" fontWeight="bold" mb={4} flexShrink={0}>Task Automation</Text>
                                     <Box flex={1} overflow="hidden">
                                         <TaskManager/>
                                     </Box>
                                 </Box>
                             )}
                         </Box>
                     </Box>
                 </Box>
            </ThemeProvider>
        </ChakraProvider>
    );
}