<?php

namespace App\Controller;

use Symfony\Bundle\FrameworkBundle\Controller\AbstractController;
use Symfony\Component\HttpFoundation\JsonResponse;
use Symfony\Component\Routing\Attribute\Route;

class UpdateController extends AbstractController
{
    #[Route('/desktop-app/{target}/{arch}/{currentVersion}', name: 'app_update')]
    public function index(string $target, string $arch, string $currentVersion): JsonResponse
    {
        return $this->json([
            'version' => 'v0.0.1',
            'notes' => 'Test version',
            'pub_date' => '2024-03-22T19:25:57Z',
            'platforms' => [
                'darwin-aarch64' => [
                    'signature' => 'dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVTaWN4OFp3dHNoS2ZGdFQvcnJaVmg5bndYSzdlNlF5dXZwSW5oaS9PemlpTWgvUjZWWHM1Sjd2ZUlFUFZLVWVhSkN4RGJmMWFFbzZQU3ozNCtCcGVORWhIMFE0TnVHbHdZPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzEwODU4MjQ4CWZpbGU6Z2FtZS1sYXVuY2hlci5hcHAudGFyLmd6Cm0zcGtWNHlQcGYzT2Z0a2l1QUJPZnRJWFlRVzJHMUNsMlVCb2dobDk2TlpxWEtNcGQzSjZjVy9Fb3FDbmcyOUNHRzhjSUt2eDlxTk9pRVVmWHFYQUJBPT0K',
                    'url' => 'https://yulbrew-game-launcher-dev.s3.ca-central-1.amazonaws.com/game-launcher.app.tar.gz',
                ],
            ],
        ]);
    }
}
