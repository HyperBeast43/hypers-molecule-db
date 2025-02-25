git push
if (-not $?)
{
    throw 'Native Failure'
}

ssh fenhl.net rustup update stable
if (-not $?)
{
    throw 'Native Failure'
}

ssh fenhl.net cargo install-update --all --git
if (-not $?)
{
    throw 'Native Failure'
}

ssh fenhl.net sudo systemctl restart molecule-db
if (-not $?)
{
    throw 'Native Failure'
}
