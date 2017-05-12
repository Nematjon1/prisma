import { AuthServer } from '../types'
import { systemAPIEndpoint, authEndpoint } from '../utils/constants'
import 'isomorphic-fetch'
import cuid = require('cuid')
import open = require('open')
const debug = require('debug')('graphcool')

export class GraphcoolAuthServer implements AuthServer {

  _projectType?: string = undefined

  constructor(projectType?: string) {
    this._projectType = projectType
  }

  async requestAuthToken(): Promise<string> {
    const cliToken = cuid()

    await fetch(`${authEndpoint}/create`, {
      method: 'post',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({cliToken}),
    })

    const frontend = 'https://cli-auth.graph.cool'
    open(`${frontend}/?token=${cliToken}`)

    while (true) {
      const url = this._projectType ?
        `${authEndpoint}/${cliToken}?projectType=${this._projectType}` : `${authEndpoint}/${cliToken}`
      console.log(`Authenticate with URL: ${url}`)
      const result = await fetch(url)

      const json = await result.json()
      const {authToken} = json
      if (authToken) {
        return authToken as string
      }
    }
  }

  async validateAuthToken(token: string) {

    const authQuery = `{
      viewer {
        user {
          id
          email
        }
      }
    }`

    try {
      const result = await fetch(systemAPIEndpoint, {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${token}`,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({query: authQuery})
      })
      const json = await result.json()

      if (!json.data.viewer.user || !json.data.viewer.user.email || json.errors) {
        return undefined
      }

      return json.data.viewer.user.email
    }
    catch (e) {
      return undefined
    }
  }

}
