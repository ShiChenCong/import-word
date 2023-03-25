import { useState } from "react";
import { open } from '@tauri-apps/api/dialog';
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [filePath, setFilePath] = useState<null | string | string[]>()
  // 上传的单词
  const [words, setWords] = useState<string[]>()
  // 第几列 
  const [columnIndex, setColumnIndex] = useState(0)
  // 上传接口需要用到的token
  const [token, setToken] = useState<string>()

  function greet() {
    // 先拿到所有的单词
    invoke("select_file", { name: filePath, key: 1 }).then(res => {
      if (res instanceof Array) {
        setWords(res)
      }
    })
  }

  function upload() {
    if (!token) {
      window.alert('请先输入token')
    }
    if (!columnIndex) {
      window.alert('请先选择上传的是第几列')
    }
    invoke('upload_word', { token }).then(res => {
      console.log(res)
      alert('success')
    }).catch(err => {
      console.log('err:', err);
    })
  }

  return (
    <div className="container">
      <div>
        <span>输入登陆后的token</span>
        <input onChange={(e) => {
          setToken(e.target.value)
        }} />
      </div>

      <div>
        <div onClick={async (e) => {
          const selected = await open({
            directory: false,
            multiple: true,
          });
          selected && setFilePath(selected)
        }} >
          选择上传的文件: {filePath ? filePath[0] : '-'}
        </div>
      </div>

      <div>
        <input onChange={(e) => {
          setColumnIndex(Number(e.target.value))
        }} />
        <div onClick={greet}>
          确认上传列
        </div>
      </div>

      <div onClick={upload}>
        开始上传
      </div>
      {/* <div onClick={() => {
        invoke('my_custom_command').then(res => {
          console.log(res)
        }).catch(err => {
          console.log('err is:', err)
        })
      }}>test</div> */}
    </div>
  );
}

export default App;
