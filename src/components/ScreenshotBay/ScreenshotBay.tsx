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
import { FaArrowLeft } from 'react-icons/fa';
import { nextWorkspaceItem } from '../../api/tauriCommands';

interface ScreenshotInfo {
    id: string;
    base64_data: string;
    width: number;
    height: number;
    timestamp: string;
    profile_id: string;
    briefcase_id: string;
    annotations: Annotation[];
}

interface Annotation {
    x: number;
    y: number;
    overlay_path: string;
}

export const ScreenshotBay = () => {
    const [screenshot, setScreenshot] = useState<ScreenshotInfo | null>(null);
    const [mousePos, setMousePos] = useState({ x: 0, y: 0 });
    const [imageSize, setImageSize] = useState({ width: 0, height: 0 });
    const [annotationCount, setAnnotationCount] = useState(0);

    useEffect(() => {
        const fetchLatestScreenshot = async () => {
            try {
                // Backend command 'tour_get_latest_screenshot' not implemented yet
                // const data = await tourInvokeGetLatestScreenshot();
                // if (data) {
                //     const parsed = JSON.parse(data);
                //     setScreenshot(parsed);
                // }
                console.warn('tour_get_latest_screenshot command not implemented in backend');
            } catch (error) {
                console.error('Failed to fetch screenshot:', error);
            }
        };

        fetchLatestScreenshot();

        const setupListener = async () => {
            const { listen } = await import('@tauri-apps/api/event');
            const unlisten = await listen('screenshot-captured', () => {
                fetchLatestScreenshot();
            });
            return unlisten;
        };

        setupListener().then(unlisten => {
            return () => unlisten();
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

        const scaleX = imageSize.width / rect.width;
        const scaleY = imageSize.height / rect.height;
        const originalX = clickX * scaleX;
        const originalY = clickY * scaleY;

        try {
            const img = new window.Image();
            img.onload = () => {
                const canvas = document.createElement('canvas');
                canvas.width = screenshot.width;
                canvas.height = screenshot.height;
                const ctx = canvas.getContext('2d');
                if (ctx) {
                    ctx.drawImage(img, 0, 0);

                    ctx.fillStyle = '#00FF00';
                    ctx.beginPath();
                    ctx.moveTo(originalX - 15, originalY);
                    ctx.lineTo(originalX, originalY + 20);
                    ctx.lineTo(originalX + 15, originalY);
                    ctx.lineTo(originalX, originalY - 10);
                    ctx.closePath();
                    ctx.fill();

                    const newBase64 = canvas.toDataURL('image/png').split(',')[1];
                    const newAnnotations = [...(screenshot.annotations || []), {
                        x: originalX,
                        y: originalY,
                        overlay_path: ''
                    }];

                    setScreenshot({
                        ...screenshot,
                        base64_data: newBase64,
                        annotations: newAnnotations
                    });
                    setAnnotationCount(prev => prev + 1);
                }
            };
            img.src = `data:image/png;base64,${screenshot.base64_data}`;
        } catch (error) {
            console.error('Failed to overlay annotation:', error);
        }
    };

    const handleFinalize = async () => {
        try {
            await nextWorkspaceItem();
        } catch (error) {
            console.error('Failed to go to next briefcase:', error);
        }
    };

    const handleGetLatest = async () => {
        try {
            // Backend command 'tour_get_latest_screenshot' not implemented yet
            // const data = await tourInvokeGetLatestScreenshot();
            // if (data) {
            //     const parsed = JSON.parse(data);
            //     setScreenshot(parsed);
            //     setAnnotationCount(0);
            // }
            console.warn('tour_get_latest_screenshot command not implemented in backend');
        } catch (error) {
            console.error('Failed to get latest screenshot:', error);
        }
    };

    return (
        <Box p={6} bg="#0a0c14" color="white" h="100vh" display="flex" flexDirection="column">
            <VStack gap={4} align="stretch" flex={1}>
                <CardRoot bg="whiteAlpha.100" border="1px solid" borderColor="whiteAlpha.200">
                    <CardBody>
                        <HStack justify="space-between" align="center">
                            <Heading size="lg">Screenshot Bay</Heading>
                            <HStack gap={3}>
                                <Button variant="outline" onClick={handleGetLatest}>
                                    Get Latest
                                </Button>
                                <Button variant="outline" onClick={handleFinalize} colorPalette="green">
                                    Finalize & Next
                                </Button>
                                <Button variant="outline" onClick={handleClose}>
                                    <FaArrowLeft style={{marginRight: '8px'}} />
                                    Back
                                </Button>
                            </HStack>
                        </HStack>
                    </CardBody>
                </CardRoot>

                {!screenshot ? (
                    <CardRoot bg="whiteAlpha.100" border="1px solid" borderColor="whiteAlpha.200" flex={1}>
                        <CardBody>
                            <VStack align="center" justify="center" h="100%" gap={4}>
                                <Text fontSize="xl" color="#718096">
                                    No screenshot available
                                </Text>
                                <Text fontSize="md" color="#4a5568" textAlign="center">
                                    Take a screenshot to begin annotation
                                </Text>
                            </VStack>
                        </CardBody>
                    </CardRoot>
                ) : (
                    <CardRoot bg="whiteAlpha.100" border="1px solid" borderColor="whiteAlpha.200" flex={1}>
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
                                    src={`data:image/png;base64,${screenshot.base64_data}`}
                                    alt="Screenshot"
                                    maxW="90%"
                                    maxH="80vh"
                                    objectFit="contain"
                                    onLoad={(e) => {
                                        setImageSize({
                                            width: e.currentTarget.naturalWidth,
                                            height: e.currentTarget.naturalHeight
                                        });
                                    }}
                                />
                                <Box
                                    position="absolute"
                                    w="20px"
                                    h="20px"
                                    bg="transparent"
                                    border="2px solid #00FF00"
                                    borderRadius="50%"
                                    left={`${(mousePos.x / (imageSize.width * 0.9)) * 100}%`}
                                    top={`${(mousePos.y / (imageSize.height * 0.8)) * 100}%`}
                                    transform="translate(-50%, -50%)"
                                    pointerEvents="none"
                                />
                            </Box>
                        </CardBody>
                    </CardRoot>
                )}

                <Box>
                    <Text fontSize="sm" color="whiteAlpha.500" textAlign="center">
                        Click to place annotation ({annotationCount} placed)
                    </Text>
                    <Text fontSize="xs" color="whiteAlpha.400" textAlign="center">
                        Click "Finalize & Next" when done to save and continue
                    </Text>
                </Box>
            </VStack>
        </Box>
    );
};
