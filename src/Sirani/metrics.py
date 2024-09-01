import os
import requests
import datetime
import argparse
import json
from elasticsearch import Elasticsearch
from elasticsearch.exceptions import ConnectionError
import urllib3

# Disable InsecureRequestWarning
urllib3.disable_warnings(urllib3.exceptions.InsecureRequestWarning)

def get_elasticsearch_client(es_host, es_port, es_user, es_password, es_use_ssl, es_verify_certs):
    # Determine the host URL
    host_url = f"{'https' if es_use_ssl else 'http'}://{es_host}:{es_port}"

    # Configure Elasticsearch client
    es_config = {
        'hosts': [host_url],
        'basic_auth': (es_user, es_password),
        'verify_certs': es_verify_certs,
        'ssl_show_warn': False
    }

    try:
        es = Elasticsearch(
            hosts=es_config['hosts'],
            basic_auth=es_config['basic_auth'],
            verify_certs=es_config['verify_certs'],
            ssl_show_warn=es_config['ssl_show_warn']
        )
        if not es.ping():
            raise ConnectionError("Failed to connect to Elasticsearch")
        print("Successfully connected to Elasticsearch")
        return es
    except ConnectionError as e:
        print(f"Failed to connect to Elasticsearch: {e}")
        print(f"Elasticsearch config: {es_config}")
        raise

def get_github_data(github_token, github_repo):
    headers = {'Authorization': f'token {github_token}'}
    github_api_url = f"https://api.github.com/repos/{github_repo}"

    def fetch_data(url):
        response = requests.get(url, headers=headers)
        if response.status_code != 200:
            print(f"Error fetching data from {url}: {response.status_code}")
            print(response.text)
            return None
        try:
            return response.json()
        except json.JSONDecodeError:
            print(f"Error decoding response from {url}")
            print(response.text)
            return None

    # Get commits
    commits = fetch_data(f"{github_api_url}/commits")
    if commits is None:
        return None
    commit_count = len(commits)

    # Get open issues
    open_issues = fetch_data(f"{github_api_url}/issues?state=open")
    if open_issues is None:
        return None
    open_issue_count = len(open_issues)

    # Get closed issues
    closed_issues = fetch_data(f"{github_api_url}/issues?state=closed")
    if closed_issues is None:
        return None
    closed_issue_count = len(closed_issues)

    # Get comments on issues
    comment_count = 0
    for issue in open_issues + closed_issues:
        if not isinstance(issue, dict):
            print(f"Unexpected issue format: {issue}")
            continue
        comments_url = issue.get('comments_url')
        if not comments_url:
            print(f"No comments_url for issue: {issue}")
            continue
        comments = fetch_data(comments_url)
        if comments is None:
            continue
        comment_count += len(comments)

    # Get open pull requests
    open_pulls = fetch_data(f"{github_api_url}/pulls?state=open")
    if open_pulls is None:
        return None
    open_pull_request_count = len(open_pulls)

    # Get closed pull requests
    closed_pulls = fetch_data(f"{github_api_url}/pulls?state=closed")
    if closed_pulls is None:
        return None
    closed_pull_request_count = len(closed_pulls)

    return {
        'timestamp': datetime.datetime.now().isoformat(),
        'commit_count': commit_count,
        'open_issue_count': open_issue_count,
        'closed_issue_count': closed_issue_count,
        'comment_count': comment_count,
        'open_pull_request_count': open_pull_request_count,
        'closed_pull_request_count': closed_pull_request_count
    }

def index_to_elasticsearch(es, data, es_index):
    if data:
        try:
            es.index(index=es_index, document=data)
            print(f"Indexed data: {data}")
        except Exception as e:
            print(f"Error indexing data to Elasticsearch: {e}")
    else:
        print("No data to index")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Fetch GitHub data and index to Elasticsearch.')
    parser.add_argument('--github_token', required=True, help='GitHub token')
    parser.add_argument('--github_repo', required=True, help='GitHub repository in the format owner/repo')
    parser.add_argument('--es_host', required=True, help='Elasticsearch host')
    parser.add_argument('--es_port', required=True, help='Elasticsearch port')
    parser.add_argument('--es_user', required=True, help='Elasticsearch user')
    parser.add_argument('--es_password', required=True, help='Elasticsearch password')
    parser.add_argument('--es_use_ssl', default='true', help='Use SSL for Elasticsearch connection')
    parser.add_argument('--es_verify_certs', default='false', help='Verify SSL certificates for Elasticsearch connection')
    parser.add_argument('--es_index', default='github_metrics', help='Elasticsearch index name')

    args = parser.parse_args()

    es_use_ssl = args.es_use_ssl.lower() == 'true'
    es_verify_certs = args.es_verify_certs.lower() == 'true'

    es = get_elasticsearch_client(args.es_host, args.es_port, args.es_user, args.es_password, es_use_ssl, es_verify_certs)

    github_data = get_github_data(args.github_token, args.github_repo)
    if github_data:
        index_to_elasticsearch(es, github_data, args.es_index)
    else:
        print("Failed to fetch GitHub data")