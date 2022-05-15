﻿namespace Platform::Data::Doublets::Memory::Split::Generic
{
    template<typename TLinksOptions>
    class ExternalLinksTargetsRecursionlessSizeBalancedTreeMethods : public ExternalLinksRecursionlessSizeBalancedTreeMethodsBase<ExternalLinksTargetsRecursionlessSizeBalancedTreeMethods<TLinksOptions>, TLinksOptions>
    {
    public: using LinksOptionsType = TLinksOptions;
                using LinkAddressType = LinksOptionsType::LinkAddressType;
        using LinkType = LinksOptionsType::LinkType;
        using WriteHandlerType = LinksOptionsType::WriteHandlerType;
        using ReadHandlerType = LinksOptionsType::ReadHandlerType;
        public: static constexpr auto Constants = LinksOptionsType::Constants;
        using base = ExternalLinksRecursionlessSizeBalancedTreeMethodsBase<ExternalLinksTargetsRecursionlessSizeBalancedTreeMethods<TLinksOptions>, TLinksOptions>;
        public: ExternalLinksTargetsRecursionlessSizeBalancedTreeMethods(std::byte* linksDataParts, std::byte* linksIndexParts, std::byte* header) : base(linksDataParts, linksIndexParts, header) { }

        public: LinkAddressType* GetLeftReference(LinkAddressType node)  { return &GetLinkIndexPartReference(node)->LeftAsTarget; }

        public: LinkAddressType* GetRightReference(LinkAddressType node)  { return &GetLinkIndexPartReference(node)->RightAsTarget; }

        public: LinkAddressType GetLeft(LinkAddressType node)  { return this->GetLinkIndexPartReference(node).LeftAsTarget; }

        public: LinkAddressType GetRight(LinkAddressType node)  { return this->GetLinkIndexPartReference(node).RightAsTarget; }

        public: void SetLeft(LinkAddressType node, LinkAddressType left)  { this->GetLinkIndexPartReference(node).LeftAsTarget = left; }

        public: void SetRight(LinkAddressType node, LinkAddressType right)  { this->GetLinkIndexPartReference(node).RightAsTarget = right; }

        public: LinkAddressType GetSize(LinkAddressType node)  { return this->GetLinkIndexPartReference(node).SizeAsTarget; }

        public: void SetSize(LinkAddressType node, LinkAddressType size)  { this->GetLinkIndexPartReference(node).SizeAsTarget = size; }

        public:  LinkAddressType GetTreeRoot() { return this->GetHeaderReference().RootAsTarget; }

        public: LinkAddressType GetBasePartValue(LinkAddressType link)  { return this->GetLinkDataPartReference(link).Target; }

        public: bool FirstIsToTheLeftOfSecond(LinkAddressType firstSource, LinkAddressType firstTarget, LinkAddressType secondSource, LinkAddressType secondTarget)  { return (firstTarget < secondTarget) || (firstTarget == secondTarget && (firstSource < secondSource)); }

        public: bool FirstIsToTheRightOfSecond(LinkAddressType firstSource, LinkAddressType firstTarget, LinkAddressType secondSource, LinkAddressType secondTarget)  { return (firstTarget > secondTarget) || (firstTarget == secondTarget && (firstSource > secondSource)); }

        public: void ClearNode(LinkAddressType node)
        {
            auto& link = this->GetLinkIndexPartReference(node);
            link.LeftAsTarget = 0;
            link.RightAsTarget = 0;
            link.SizeAsTarget = 0;
        }
    };
}
