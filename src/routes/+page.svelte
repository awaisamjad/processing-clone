<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { appWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';
	import { open } from '@tauri-apps/api/dialog';
	import { readTextFile } from '@tauri-apps/api/fs';
	import {
		AppShell,
		CodeBlock,
		AppBar,
		LightSwitch,
		popup,
		getModalStore
	} from '@skeletonlabs/skeleton';
	let code = '';

	onMount(() => {
		appWindow.listen('open_file', async () => {
			try {
				const filePath = await open({
					multiple: false,
					filters: [
						{
							name: 'Awa File',
							extensions: ['awa']
						}
					]
				});
				if (filePath === undefined) {
					return;
				}
				const file_contents = await readTextFile(filePath, {});
				code = file_contents;
				//TODO ->invoke('send_code', {code: code});
				console.log(file_contents);
			} catch (error) {
				console.error(error);
			}
		});
	});

	// async function openAwaFile() {
	// 	const file = await invoke('open_awa_file', {awaFileCode});
	// 	console.log(file);
	// }
	// // openAwaFile();
	// console.log('awaFileCode : ', awaFileCode);
</script>

<AppBar>
	<button type="button" class="btn-icon variant-filled">Run</button>
	<button type="button" class="btn-icon bg-gradient-to-br variant-gradient-secondary-primary"
		>Stop</button
	>
</AppBar>

<textarea bind:value={code} style="color : black; width: 100%; height: 100%; caret-color: red;"
></textarea>

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
  value={"hello world"}
/> -->

<style>
</style>
