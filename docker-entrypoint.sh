#!/bin/bash
# 入口脚本：优先使用 /data/config.toml，否则根据环境变量生成 config.toml

set -e

APP_DIR=/app
CFG_PATH="${APP_DIR}/config.toml"

# 如果 /data/config.toml 存在，则拷贝到容器内
if [ -f "/data/config.toml" ]; then
  echo "检测到 /data/config.toml，拷贝到 ${CFG_PATH}"
  cp /data/config.toml "${CFG_PATH}"
else
  echo "未检测到 /data/config.toml，根据环境变量生成 config.toml"
  cat > "${CFG_PATH}" <<EOF
check_update = ${CHECK_UPDATE:-true}
auto_update = ${AUTO_UPDATE:-false}
max_retries = ${MAX_RETRIES:-5}
wasted_cookie = []
password = "${PASSWORD:-changeme}"
proxy = "${PROXY:-}"
ip = "${IP:-0.0.0.0}"
port = ${PORT:-8484}
enable_oai = ${ENABLE_OAI:-false}
pass_params = ${PASS_PARAMS:-false}
preserve_chats = ${PRESERVE_CHATS:-false}
skip_warning = ${SKIP_WARNING:-false}
skip_restricted = ${SKIP_RESTRICTED:-false}
skip_non_pro = ${SKIP_NON_PRO:-false}
rproxy = "${RPROXY:-}"
use_real_roles = ${USE_REAL_ROLES:-true}
custom_prompt = "${CUSTOM_PROMPT:-}"
padtxt_file = "${PADTXT_FILE:-}"
padtxt_len = ${PADTXT_LEN:-4000}

[[cookie_array]]
cookie = "${COOKIE1:-sessionKey=sk-ant-sid01-SET_YOUR_COOKIE_HERE-AAAAAAAA}"

[[cookie_array]]
cookie = "${COOKIE2:-sessionKey=sk-ant-sid01-SET_YOUR_COOKIE_HERE-AAAAAAAA}"
discord = "${DISCORD2:-}"
due = ${DUE2:-114514000}
EOF
fi

# 启动主程序
exec clewdr
