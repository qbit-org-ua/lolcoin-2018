import React from 'react'

import Link from 'next/link'


export default class extends React.Component {
  render() {
    return (
      <div className="container">
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>ЛОЛкоин</title>
        <link data-react-helmet="true" rel="stylesheet" href="/static/css/bootstrap.min.css"/>

        <div className="nav">
          <div className="navbar">
            <Link href={{ pathname: '/' }}><a className="nav-link">Главная страница</a></Link>
            <Link href={{ pathname: '/transfers' }}><a className="nav-link">История переводов</a></Link>
          </div>
        </div>
        {this.props.children}
      </div>
    )
  }
}
