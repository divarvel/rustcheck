guard :shell do
  watch("Gemfile") do |m|
    title = "Bundler output"
    msg = "Bundler Failure"
    status = :failed

    if system("bundle")
      msg = "Bundled"
      status = :status
    end

    n msg, title, status

    "-> #{msg}"
  end

  watch(/\.rs/) do |m|
    title = "Make output"
    msg = "Compilation failure"
    status = :failed

    if system("make")
      msg = "Tested"
      status = :success
    end

    n msg, title, status

    "-> #{msg}"
  end
end
