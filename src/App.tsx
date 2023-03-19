import { useState } from "react";
import { open } from '@tauri-apps/api/dialog';
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [selectedPath, setSelectedPath] = useState<null | string | string[]>()
  const [words, setWords] = useState<string[]>()
  const [token, setToken] = useState<string>()

  function greet() {
    // 先拿到所有的单词
    invoke("select_file", { name: selectedPath }).then(res => {
      if (res instanceof Array) {
        setWords(res)
      }
    })
  }

  function upload() {
    if (!token) {
      window.alert('请先输入token')
    } else {
      document.cookie = token;
    }
    const params = {
      business_id: 6,
      words: ['feast'],
    }
    fetch("https://apiv3.shanbay.com/wordscollection/words_bulk_upload", {
      method: 'post',
      credentials: 'include',
      body: JSON.stringify(params)
    })
  }

  return (
    <div className="container">
      <div>
        <span>输入登陆后的token</span>
        <input onChange={(e) => {
          console.log(e.target.value)
          setToken(e.target.value)
        }} />
      </div>
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
