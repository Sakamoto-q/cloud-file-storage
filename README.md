> [!WARNING]
> This project requires **S3 storage**, **TURNSTILE**, and **MySQL**.
> Please configure your `.env` file according to the `.env.example` before running.
> Additionally, since presigned URLs are used with S3, **CORS must be properly configured**.

```sh
docker build -t cloud-file-storage .
```

```sh
docker run -p 9000:9000 --restart unless-stopped -d cloud-file-storage
```