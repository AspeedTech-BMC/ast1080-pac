#[doc = "Register `JTAG02C` reader"]
pub type R = crate::R<Jtag02cSpec>;
#[doc = "Register `JTAG02C` writer"]
pub type W = crate::W<Jtag02cSpec>;
#[doc = "Field `PrePaddingNumber` reader - Pre Padding Number"]
pub type PrePaddingNumberR = crate::FieldReader<u16>;
#[doc = "Field `PrePaddingNumber` writer - Pre Padding Number"]
pub type PrePaddingNumberW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PostPaddingNumber` reader - Post Padding Number"]
pub type PostPaddingNumberR = crate::FieldReader<u16>;
#[doc = "Field `PostPaddingNumber` writer - Post Padding Number"]
pub type PostPaddingNumberW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PaddingData` reader - Padding Data"]
pub type PaddingDataR = crate::BitReader;
#[doc = "Field `PaddingData` writer - Padding Data"]
pub type PaddingDataW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Pre Padding Number"]
    #[inline(always)]
    pub fn pre_padding_number(&self) -> PrePaddingNumberR {
        PrePaddingNumberR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:11 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:20 - Post Padding Number"]
    #[inline(always)]
    pub fn post_padding_number(&self) -> PostPaddingNumberR {
        PostPaddingNumberR::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bits 21:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - Padding Data"]
    #[inline(always)]
    pub fn padding_data(&self) -> PaddingDataR {
        PaddingDataR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Pre Padding Number"]
    #[inline(always)]
    pub fn pre_padding_number(&mut self) -> PrePaddingNumberW<Jtag02cSpec> {
        PrePaddingNumberW::new(self, 0)
    }
    #[doc = "Bits 9:11 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Jtag02cSpec> {
        Reserved2W::new(self, 9)
    }
    #[doc = "Bits 12:20 - Post Padding Number"]
    #[inline(always)]
    pub fn post_padding_number(&mut self) -> PostPaddingNumberW<Jtag02cSpec> {
        PostPaddingNumberW::new(self, 12)
    }
    #[doc = "Bits 21:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Jtag02cSpec> {
        Reserved1W::new(self, 21)
    }
    #[doc = "Bit 24 - Padding Data"]
    #[inline(always)]
    pub fn padding_data(&mut self) -> PaddingDataW<Jtag02cSpec> {
        PaddingDataW::new(self, 24)
    }
}
#[doc = "Padding control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag02c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag02c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jtag02cSpec;
impl crate::RegisterSpec for Jtag02cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtag02c::R`](R) reader structure"]
impl crate::Readable for Jtag02cSpec {}
#[doc = "`write(|w| ..)` method takes [`jtag02c::W`](W) writer structure"]
impl crate::Writable for Jtag02cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JTAG02C to value 0"]
impl crate::Resettable for Jtag02cSpec {}
