// Quick test to verify Tauri commands work
import { loadProfiles, saveProfiles, addHistoryEntry } from './src/api/tauriCommands.js';

async function testCommands() {
    try {
        console.log('Testing Tauri commands...');
        
        // Test load profiles
        const profiles = await loadProfiles();
        console.log('Loaded profiles:', profiles);
        
        // Test history
        await addHistoryEntry('test', 'test-id', 'created', 'Test entry');
        console.log('History entry added');
        
        console.log('All commands working!');
    } catch (error) {
        console.error('Command test failed:', error);
    }
}

if (typeof window !== 'undefined') {
    // Only run in browser environment
    testCommands();
}