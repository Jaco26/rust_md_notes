import DirectoryItems from './directory-items.js'

export default {
  // functional: true,
  name: 'Directory',
  props: {
    name: String,
    child_dirs: Array,
    files: Array,
  },
  render(h, ctx) {
    console.log(this.$props)
    return h(
      'div',
      null, 
      [
        this.$props.name,
        h(
          DirectoryItems,
          {
            props: {
              child_dirs: this.$props.child_dirs,
              files: this.$props.files,
            }
          },
        )
      ]
    )
  }
}