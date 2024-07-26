OWNER="HeliosShieldProject"
REPO="agent-wireguard-rust"
SAVE_PATH="asset.tar.gz"

rm -f "$SAVE_PATH" agent-wireguard download_binary.sh

if pgrep -x "agent-wireguard" > /dev/null
then
  pkill agent-wireguard
fi

ASSET_URL=$(curl -s -H "Accept: application/vnd.github.v3+json" https://api.github.com/repos/$OWNER/$REPO/releases/latest | grep '"browser_download_url"' | grep 'tar.gz' | head -n 1 | sed -E 's/.*"([^"]+)".*/\1/')
if [ -z "$ASSET_URL" ]; then
  echo "No assets found in the latest release."
  exit 1
fi

curl -L -o "$SAVE_PATH" "$ASSET_URL"
tar -xzf "$SAVE_PATH"

rm -f "$SAVE_PATH"
