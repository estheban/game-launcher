<?php

namespace App\EventListener;

use Symfony\Component\HttpFoundation\JsonResponse;
use Symfony\Component\HttpKernel\Event\ExceptionEvent;
use Symfony\Component\HttpKernel\Exception\HttpException;

class JsonExceptionListener
{
    public function onKernelException(ExceptionEvent $event): void
    {
        // You get the exception object from the received event
        $exception = $event->getThrowable();
        $message = sprintf(
            'Error: %s with code: %s',
            $exception->getMessage(),
            $exception->getCode()
        );

        $statusCode = JsonResponse::HTTP_INTERNAL_SERVER_ERROR;
        if ($exception instanceof HttpException) {
            $statusCode = $exception->getStatusCode();
        }

        // Customize your response object to display the exception details
        $response = new JsonResponse(
            ['error' => $message, 'event' => $event],
            $statusCode
        );

        // Sends the modified response object to the event
        $event->setResponse($response);
    }
}
