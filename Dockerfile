# 基础镜像，适用于本地和 CI 构建
FROM rustlang/rust:nightly as builder

# 设置构建参数
ARG APP_DIR=/app

# 创建工作目录
WORKDIR ${APP_DIR}

# 安装常见构建依赖（如 openssl、sqlite、zlib）
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*

# 复制项目文件
COPY . .

# 构建 release 版本
RUN cargo build --release

# 运行阶段，使用更小的基础镜像
FROM debian:bullseye-slim

ARG APP_DIR=/app

# 安装运行时依赖
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# 创建工作目录
WORKDIR ${APP_DIR}

# 从构建镜像复制可执行文件
COPY --from=builder ${APP_DIR}/target/release/clewdr /usr/local/bin/clewdr

# 复制静态资源（如有前端静态文件，可根据实际情况调整）
COPY frontend/public ./frontend/public

# 默认暴露端口（可根据 config.toml 配置调整）
EXPOSE 8484

# 默认启动命令
CMD ["clewdr"]
