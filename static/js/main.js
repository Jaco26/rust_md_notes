import './vendor/vue.js'
import { api, buildFileTreeRecursively } from './util.js'

import Directory from './components/directory.js'



const app = new Vue({
  components: {
    Directory
  },
  computed: {
    _dirList() {
      return Object.values(this.dirs)
    },
    rootDir() {
      return this._dirList.find(dir => dir.name === 'root') || {}
    },
    dirTree() {
      return buildFileTreeRecursively({}, this.dirs, this.rootDir)
    }
  },
  data() {
    return {
      newFileName: '',
      dirs: {},
      errors: []
    }
  },
  methods: {
    async init() {
      try {
        this.dirs = {}
        const { data } = await api.doGet('/dir/list')
        data.forEach(dir => this.fetchDir(dir.id))
      } catch (error) {
        this.errors.push(error)
      }
    },
    async fetchDir(dirId) {
      try {
        const { data } = await api.doGet(`/dir/${dirId}`)
        this.$set(this.dirs, dirId, data)
      } catch (error) {
        this.errors.push(error)
      }
    },
    async onCreateNewFile(parentDirId) {
      try {
        await api.doPost('/file/', {
          name: this.newFileName,
          parent_dir_id: parentDirId
        })
        this.init()
      } catch (error) {
        console.error(error)
        this.errors.push(error)
      }
    }
  },
  mounted() {
    this.init()
  },
  template: `
    <div id="app">
      <div class="page">
        <div class="page__file-tree">
          <div>
            <form @submit.prevent="onCreateNewFile(rootDir.id)">
              <div class="input">
                <label class="input__label">New File</label>
                <input class="input__input" type="text" v-model="newFileName" />
              </div>
              <button type="submit">Submit</button>
            </form>
          </div>
          <div>
            <div class="dir-tree" v-if="rootDir.id">
              <Directory v-bind="dirTree[rootDir.id]" />
              <!-- <Directory
                v-if="rootDir.id"
                :child_dirs="dirTree[rootDir.id].child_dirs"
                :files="dirTree[rootDir.id].files"
              /> -->
            </div>
          </div>
        </div>
        <div class="page__main-content">
          main content
          <pre>
            {{dirTree}}
          </pre>
        </div>
      </div>
    </div>
  `,
})

app.$mount('#app')


