import { useState } from "react";
import { open } from '@tauri-apps/api/dialog';
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [selectedPath, setSelectedPath] = useState<null | string | string[]>()
  const [words, setWords] = useState<string[]>()

  // 不能放在body前面 会堵塞页面渲染
  window.AWSC.use("nvc", function(state, module) {
    window.nvc = module.init({
      appkey: "FFFF0N0N000000007037",
      scene: "nvc_message_h5",
    });
  });

  function greet() {
    // 先拿到所有的单词
    invoke("select_file", { name: selectedPath }).then(res => {
      if (res instanceof Array) {
        setWords(res)
      }
    })
  }

  function upload() {
    const params = {
      "business_id": 6, "words": words
    }
    fetch("https://apiv3.shanbay.com/bayuser/login", {
      method: 'POST',
      headers: {
        "x-api-afs-nvc": window.nvc.getNVCVal()
      },
      body: JSON.stringify({ "account": "17369669007", "password": "karl463848340" })
    }).then(res => res.json()).then(res => {
      console.log('res: ', res)
    })
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
        确认单词
      </div>
      <div onClick={upload}>
        开始上传
      </div>
    </div>
  );
}

export default App;
