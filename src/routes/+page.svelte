<script>
	import { invoke } from '@tauri-apps/api/tauri'
	import {
		AppShell,
		CodeBlock,
		AppBar,
		LightSwitch,
		popup,
		getModalStore
	} from '@skeletonlabs/skeleton';
	// import { AceEditor } from 'svelte-ace';
	// import 'brace/mode/json';
	// import 'brace/theme/chrome';
	let code = '';
	async function getCode() {
		return await invoke('get_code', { code }).then((res) => {
			console.log(res);
		});
	}

	async function stop() {
		return await invoke('stop_code', { code }).then((res) => {
			console.log(res);
		});
	}

	async function sendAwaFile() {
		const file = await invoke('send_awa_file');
		console.log(file);
	}
	sendAwaFile();
</script>

<AppBar>
	<button type="button" class="btn-icon variant-filled" on:click={getCode}>Run</button>
	<button type="button" class="btn-icon bg-gradient-to-br variant-gradient-secondary-primary"
		>Stop</button
	>
</AppBar>
<textarea bind:value={code} style="color : black"></textarea>
<!-- <AceEditor
on:selectionChange={(obj) => console.log(obj.detail)}
  on:paste={(obj) => console.log(obj.detail)}
  on:input={(obj) => console.log(obj.detail)}
  on:focus={() => console.log('focus')}
  on:documentChange={(obj) => console.log(`document change : ${obj.detail}`)}
  on:cut={() => console.log('cut')}
  on:cursorChange={() => console.log('cursor change')}
  on:copy={() => console.log('copy')}
  on:init={(editor) => console.log(editor.detail)}
  on:commandKey={(obj) => console.log(obj.detail)}
  on:changeMode={(obj) => console.log(`change mode : ${obj.detail}`)}
  on:blur={() => console.log('blur')}
  width='100%'
  height='300px'
  lang="json"
  theme="chrome"
  value={text}
/> -->
