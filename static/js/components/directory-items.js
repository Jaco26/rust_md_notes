import Directory from './directory.js'

export default {
  // functional: true,
  name: 'DirectoryItems',
  props: {
    child_dirs: Array,
    files: Array,
  },

  render(h, ctx) {
    const files = this.$props.files.map(file => (
      h(
        'li',
        {
          class: 'dir-items__file',
          on: {
            click: () => {
              console.log('You clicked this file!', file)
            }
          }
        },
        file.name
      )
    ))

    const childDirs = Object.values(this.$props.child_dirs)
      .reduce((acc, dir) => {
        const [id] = Object.keys(dir)
        acc[id] = dir[id]
        return acc
      }, {})

    console.log('childDirs', Object.values(childDirs))

    const dirs = Object.values(childDirs).map(dir => (
      h(
        'li',
        {
          class: 'dir-items__child-dir',
          on: {
            click: () => {
              console.log('you clicked this directory!', dir)
            }
          }
        },
        [
          dir.name,
          ...dir.child_dirs.map(childDir => (
            h(
              Directory,
              {
                props: {
                  files: childDir.files,
                  name: childDir.name,
                  child_dirs: childDir.child_dirs
                }
              }
            )
          ))
        ]
      )
    ))
    return h('ul', null, files.concat(dirs))
  }
}