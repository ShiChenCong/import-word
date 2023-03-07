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
    // fetch("http://127.0.0.1:8080/a/scc")
    //   .then((response) => response.json())
    //   .then((data) => console.log(data));
    const params = {
      "business_id": 6, "words": words
    }
    fetch("http://127.0.0.1:8080", {
      method: 'post',
      headers: {
        // "x-api-afs-nvc": window.nvc.getNVCVal()
        "Content-type": "application/json",
      },
      // body: JSON.stringify({ "id": 123, "name": "scc" })
      body: JSON.stringify({ nvc: window.nvc.getNVCVal() })
    }).then(res => res.json()).then(res => {
      console.log('res: ', res)
    }).catch(err => {
      console.log('err: ', err)
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
