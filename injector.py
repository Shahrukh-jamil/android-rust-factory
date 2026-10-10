import sys, os, requests, json, shutil

if len(sys.argv) < 2:
    print("Error: No Gist ID provided.")
    sys.exit(1)

GIST_ID = sys.argv[1]
print(f"Fetching Gist: {GIST_ID}")
response = requests.get(f"https://api.github.com/gists/{GIST_ID}")
files = response.json().get('files', {})

config = {
    "appName": "RustApp",
    "packageId": "com.factory.rustapp",
    "versionCode": 1,
    "orientation": "portrait",
    "permissions": []
}

if "config.json" in files:
    config.update(json.loads(files["config.json"]["content"]))

app_name = config["appName"]
pkg_id = config["packageId"]
v_code = str(config.get("versionCode", 1))
v_name = f"1.0.{v_code}"
orientation = config.get("orientation", "portrait")
permissions = config.get("permissions", [])

env_file = os.getenv('GITHUB_ENV')
if env_file:
    with open(env_file, 'a') as f:
        safe_name = app_name.replace(' ', '')
        f.write(f"APP_NAME={safe_name}\n")
        f.write(f"VERSION_CODE={v_code}\n")
        f.write(f"VERSION_NAME={v_name}\n")

print(f"Injecting App: {app_name} ({pkg_id}) v{v_code}")

# 1. Generate Manifest with Target SDK 34 & Version Code
perm_lines = [f'    <uses-permission android:name="{p}" />' for p in permissions]
perms_block = "\n".join(perm_lines)

with open("AndroidManifest.xml.template", "r") as f:
    template = f.read()

manifest = template.format(
    package_id=pkg_id,
    version_code=v_code,
    version_name=v_name,
    permissions_block=perms_block,
    orientation=orientation
)

with open("AndroidManifest.xml", "w") as f:
    f.write(manifest)

# 2. Update Strings XML
strings_dir = "res/values"
os.makedirs(strings_dir, exist_ok=True)
with open(os.path.join(strings_dir, "strings.xml"), "w") as f:
    f.write(f'<?xml version="1.0" encoding="utf-8"?>\n<resources>\n    <string name="app_name">{app_name}</string>\n</resources>')

# 3. Inject Source Files & Vectors
for filename, file_data in files.items():
    content = file_data["content"]
    if filename.endswith(".rs"):
        with open(os.path.join("src", filename), "w") as f:
            f.write(content)
    elif filename == "app_icon.xml":
        draw_dir = "res/drawable"
        os.makedirs(draw_dir, exist_ok=True)
        with open(os.path.join(draw_dir, "ic_launcher.xml"), "w") as f:
            f.write(content)

print("Injection Complete!")
