> [!WARNING]
> 이 프로젝트를 실행하려면 **S3 스토리지**, **TURNSTILE**, **MySQL**이 필요합니다.
> 실행 전에 `.env` 파일을 `.env.example` 파일에 맞게 설정하세요.
> 또한, S3 스토리지에서 presigned URL을 사용하므로 **CORS 설정**이 필요합니다.

```sh
docker build -t cloud-file-storage .
```

```sh
docker run -p 9000:9000 --restart unless-stopped -d cloud-file-storage
```