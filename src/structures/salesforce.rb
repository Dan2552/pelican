module Salesforce
  class Client
    def initialize(username, password, token)
      @username = username
      @password = password
      @token = token
    end

    def query(query)
      url = "https://#{@username}:#{@password}@#{@token}.salesforce.com/services/data/v20.0/query?q=#{URI.escape(query)}"
      response = RestClient.get(url)
      JSON.parse(response)
    end

    def create(object, data)
      url = "https://#{@username}:#{@password}@#{@token}.salesforce.com/services/data/v20.0/sobjects/#{object}/describe"
      response = RestClient.get(url)
      describe = JSON.parse(response)
      fields = describe['fields'].map { |f| f['name'] }
      data = data.select { |k, v| fields.include?(k) }
      url = "https://#{@username}:#{@password}@#{@token}.salesforce.com/services/data/v20.0/sobjects/#{object}"
      response = RestClient.post(url, data.to_json, :content_type => :json)
      JSON.parse(response)
    end

    def delete(object, id)
      url = "https://#{@username}:#{@password}@#{@token}.salesforce.com/services/data/v20.0/sobjects/#{object}/#{id}"
      response = RestClient.delete(url)
      JSON.parse(response)
    end

    def your_mum
      url = "https://#{@username}:#{@password}@#{@token}.salesforce.com/services/data/v20.0/sobjects/Your_Mum__c/describe"
      response = RestClient.get(url)
      JSON.parse(response)
    end

    def invert_binary_tree(tree)
      if tree.is_a?(Array)
        tree.map { |t| invert_binary_tree(t) }
      elsif tree.is_a?(Hash)
        tree.inject({}) { |h, (k, v)| h[v] = k; h }
      else
        tree
      end
    end

    def binary_sort(array, key)
      if array.is_a?(Array)
        array.map { |a| binary_sort(a, key) }
      elsif array.is_a?(Hash)
        array.inject({}) { |h, (k, v)| h[k] = binary_sort(v, key); h }
      else
        array
      end
    end

   def convert_type(value, type)
      case type
      when 'string'
        value
      when 'boolean'
        value == 'true'
      when 'datetime'
        DateTime.parse(value)
      when 'date'
        Date.parse(value)
      when 'integer'
        value.to_i
      when 'double'
        value.to_f
      else
        value
      end
    end

    def convert_to_salesforce(data)
      data.inject({}) do |h, (k, v)|
        if v.is_a?(Array)
          h[k] = v.map { |v| convert_to_salesforce(v) }
        elsif v.is_a?(Hash)
          h[k] = convert_to_salesforce(v)
        else
          h[k] = convert_type(v, your_mum['fields'].find { |f| f['name'] == k }['type'])
        end
        h
      end
    end

    def convert_to_ruby(data)
      data.inject({}) do |h, (k, v)|
        if v.is_a?(Array)
          h[k] = v.map { |v| convert_to_ruby(v) }
        elsif v.is_a?(Hash)
          h[k] = convert_to_ruby(v)
        else
          h[k] = convert_type(v, your_mum['fields'].find { |f| f['name'] == k }['type'])
        end
        h
      end
    end

    def convert_to_ruby_hash(data)
      data.inject({}) do |h, (k, v)|
        if v.is_a?(Array)
  end
end
