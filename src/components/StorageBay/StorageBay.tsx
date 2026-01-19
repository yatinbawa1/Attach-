import { useEffect, useState } from 'react';
import {
    Box,
    Button,
    CardRoot,
    CardBody,
    Heading,
    HStack,
    Image,
    Text,
    VStack
} from '@chakra-ui/react';
import { FaArrowLeft, FaTimes } from 'react-icons/fa';

interface ScreenshotInfo {
    id: string;
    dataUrl: string;
    width: number;
    height: number;
}

export const ScreenshotAnnotationBay = () => {
    const [screenshot, setScreenshot] = useState<ScreenshotInfo | null>(null);
    const [mousePos, setMousePos] = useState({ x: 0, y: 0 });
    const [imageSize, setImageSize] = useState({ width: 0, height: 0 });
    const [imageKey, setImageKey] = useState(0);

    useEffect(() => {
        console.log('StorageBay mounted, setting up listener');
        // Listen for screenshot from webview
        const setupListener = async () => {
            const { listen } = await import('@tauri-apps/api/event');
            const unlisten = await listen('screenshot-from-webview', (event: any) => {
                console.log('Received screenshot-from-webview event');
                const data = event.payload as { dataUrl: string };
                if (data && data.dataUrl) {
                    console.log('Screenshot data received, creating image');
                    // Create an image to get dimensions
                    const img = new window.Image();
                    img.onload = () => {
                        console.log('Image loaded, dimensions:', img.width, img.height);
                        setScreenshot({
                            id: 'screenshot',
                            dataUrl: data.dataUrl,
                            width: img.width,
                            height: img.height
                        });
                    };
                    img.onerror = () => {
                        console.error('Failed to load image from data URL');
                    };
                    img.src = data.dataUrl;
                } else {
                    console.log('No dataUrl in payload:', data);
                }
            });

            return unlisten;
        };

        setupListener().then(unlisten => {
            console.log('Listener set up successfully');
            return () => {
                console.log('Cleaning up listener');
                unlisten();
            };
        });
    }, []);

    const handleClose = async () => {
        const { getCurrentWindow } = await import('@tauri-apps/api/window');
        getCurrentWindow().close();
    };

    const handleMouseMove = (e: React.MouseEvent<HTMLDivElement>) => {
        const rect = e.currentTarget.getBoundingClientRect();
        setMousePos({
            x: e.clientX - rect.left,
            y: e.clientY - rect.top
        });
    };

    const handleClick = async (e: React.MouseEvent<HTMLDivElement>) => {
        if (!screenshot) return;

        const rect = e.currentTarget.getBoundingClientRect();
        const clickX = e.clientX - rect.left;
        const clickY = e.clientY - rect.top;

        // Scale to original image size
        const scaleX = imageSize.width / rect.width;
        const scaleY = imageSize.height / rect.height;
        const originalX = clickX * scaleX;
        const originalY = clickY * scaleY;

        try {
            // For now, just show the click worked
            console.log('Click at:', originalX, originalY);

            // In a real implementation, we'd send the position to backend for overlay
            // For now, we can draw on a canvas and update the image
            const img = new window.Image();
            img.onload = () => {
                const canvas = document.createElement('canvas');
                canvas.width = screenshot.width;
                canvas.height = screenshot.height;
                const ctx = canvas.getContext('2d');
                if (ctx) {
                    ctx.drawImage(img, 0, 0);
                    // Draw a tick at the clicked position
                    ctx.fillStyle = 'green';
                    ctx.beginPath();
                    ctx.moveTo(originalX - 20, originalY);
                    ctx.lineTo(originalX, originalY + 20);
                    ctx.lineTo(originalX + 20, originalY);
                    ctx.lineTo(originalX, originalY - 10);
                    ctx.closePath();
                    ctx.fill();

                    setScreenshot({
                        ...screenshot,
                        dataUrl: canvas.toDataURL('image/png')
                    });
                    setImageKey(prev => prev + 1);
                }
            };
            img.src = screenshot.dataUrl;
        } catch (error) {
            console.error('Failed to overlay tick:', error);
        }
    };

    return (
        <Box p={6} bg="#0a0c14" color="white" h="100vh" display="flex" flexDirection="column">
            <VStack gap={6} align="stretch" maxW="100%" flex={1}>
                {/* Header */}
                <CardRoot bg="whiteAlpha.100" backdropFilter="blur(10px)" border="1px solid" borderColor="whiteAlpha.200">
                    <CardBody>
                        <HStack justify="space-between" align="center">
                            <Heading size="lg">Storage Bay</Heading>
                            <HStack gap={3}>
                                <Button
                                    variant="outline"
                                    onClick={handleClose}
                                >
                                    <FaArrowLeft style={{marginRight: '8px'}} />
                                    Back to Work
                                </Button>
                                <Button
                                    variant="outline"
                                    onClick={handleClose}
                                >
                                    <FaTimes style={{marginRight: '8px'}} />
                                    Close
                                </Button>
                            </HStack>
                        </HStack>
                    </CardBody>
                </CardRoot>

                {/* Screenshot Display */}
                {!screenshot ? (
                    <CardRoot bg="whiteAlpha.100" backdropFilter="blur(10px)" border="1px solid" borderColor="whiteAlpha.200" flex={1}>
                        <CardBody>
                            <VStack align="center" justify="center" h="100%" gap={4}>
                                <Text fontSize="xl" color="#718096">
                                    No screenshot available
                                </Text>
                                <Text fontSize="md" color="#4a5568" textAlign="center">
                                    Take a screenshot first
                                </Text>
                            </VStack>
                        </CardBody>
                    </CardRoot>
                ) : (
                    <CardRoot bg="whiteAlpha.100" backdropFilter="blur(10px)" border="1px solid" borderColor="whiteAlpha.200" flex={1}>
                        <CardBody display="flex" alignItems="center" justifyContent="center" h="100%">
                            <Box
                                position="relative"
                                border="2px solid"
                                borderColor="blue.400"
                                borderRadius="md"
                                overflow="hidden"
                                cursor="crosshair"
                                onMouseMove={handleMouseMove}
                                onClick={handleClick}
                                display="inline-block"
                            >
                                <Image
                                    key={imageKey}
                                    src={screenshot.dataUrl}
                                    alt="Screenshot"
                                    maxW="80%"
                                    maxH="80%"
                                    objectFit="contain"
                                    onLoad={(e) => {
                                        setImageSize({
                                            width: e.currentTarget.naturalWidth,
                                            height: e.currentTarget.naturalHeight
                                        });
                                    }}
                                />
                                {/* Mouse position indicator */}
                                <Box
                                    position="absolute"
                                    w="4px"
                                    h="4px"
                                    bg="red.500"
                                    borderRadius="50%"
                                    left={`${(mousePos.x / (imageSize.width * 0.2)) * 100}%`}
                                    top={`${(mousePos.y / (imageSize.height * 0.2)) * 100}%`}
                                    pointerEvents="none"
                                    transform="translate(-50%, -50%)"
                                />
                            </Box>
                        </CardBody>
                    </CardRoot>
                )}

                {/* Info */}
                <Box>
                    <Text fontSize="xs" color="whiteAlpha.500" textAlign="center">
                        Click on the image to place a tick mark
                    </Text>
                </Box>
            </VStack>
        </Box>
    );
};