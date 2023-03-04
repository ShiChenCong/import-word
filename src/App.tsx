import { useState } from "react";
import { open } from '@tauri-apps/api/dialog';
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [selectedPath, setSelectedPath] = useState<null | string | string[]>()

  window.AWSC.use("nvc", function(state, module) {
    window.nvc = module.init({
      appkey: "FFFF0N0N000000007037",
      scene: "nvc_message_h5",
    });
  });

  async function greet() {
    // const result = await invoke("select_file", { name: selectedPath })
    // console.log('result: ', result)
    const a = window.nvc.getNVCVal()
    console.log('a: ', a)
  }

  return (
    <div className="container">
      <div onClick={async (e) => {
        const selected = await open({
          directory: false,
          multiple: true,
        });
        selected && setSelectedPath(selected)
      }} >
        选择路径(支持多选)
      </div>
      <div onClick={greet}>
        确认
      </div>
    </div>
  );
}

export default App;
