import { useState } from "react";
import { open } from '@tauri-apps/api/dialog';
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { sendNotification } from "@tauri-apps/api/notification"

function App() {
  const [filePath, setFilePath] = useState<null | string | string[]>()
  // 第几列 
  const [columnIndex, setColumnIndex] = useState(0)
  // 上传接口需要用到的token
  const [token, setToken] = useState<string>()

  function upload() {
    // window.alert(123)
    sendNotification({ title: 'TAURI', body: 'Tauri is awesome!', icon: 'icon' });
    // if (!token) {
    //   window.alert('请先输入token')
    // }
    // if (!columnIndex) {
    //   window.alert('请先选择上传的是第几列')
    // }
    invoke('upload_word', { token, filePath, uploadIndex: columnIndex }).then(res => {
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
          const files = await open({
            directory: false,
            multiple: true,
            // 加上标题速度能快不少
            title: '选择文件'
          });
          files && setFilePath(files)
        }} >
          选择上传的文件: {filePath ? filePath[0] : '-'}
        </div>
      </div>

      <div>
        <div>输入上传列</div>
        <input onChange={(e) => {
          setColumnIndex(Number(e.target.value))
        }} />
        {/* <div onClick={greet}>
          确认上传列
        </div> */}
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
