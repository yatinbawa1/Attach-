import {useEffect} from 'react';
import {toaster} from "@/components/ui/toaster"; // Ensure you ran: npx @chakra-ui/cli snippet add toaster
import {useStore} from '../../store';

export const ErrorSystem = () => {
    const {error, setError} = useStore();

    useEffect(() => {
        if (error) {
            toaster.create({
                title: "System Error",
                description: error,
                type: "error",
                duration: 5000,
            });
            
            // Clear error after toast duration
            const timer = setTimeout(() => {
                setError(null);
            }, 5000);
            
            return () => clearTimeout(timer);
        }
    }, [error, setError]);

    return null;
};