#[doc = "Register `JTAG028` reader"]
pub type R = crate::R<Jtag028Spec>;
#[doc = "Register `JTAG028` writer"]
pub type W = crate::W<Jtag028Spec>;
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
    pub fn pre_padding_number(&mut self) -> PrePaddingNumberW<Jtag028Spec> {
        PrePaddingNumberW::new(self, 0)
    }
    #[doc = "Bits 9:11 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Jtag028Spec> {
        Reserved2W::new(self, 9)
    }
    #[doc = "Bits 12:20 - Post Padding Number"]
    #[inline(always)]
    pub fn post_padding_number(&mut self) -> PostPaddingNumberW<Jtag028Spec> {
        PostPaddingNumberW::new(self, 12)
    }
    #[doc = "Bits 21:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Jtag028Spec> {
        Reserved1W::new(self, 21)
    }
    #[doc = "Bit 24 - Padding Data"]
    #[inline(always)]
    pub fn padding_data(&mut self) -> PaddingDataW<Jtag028Spec> {
        PaddingDataW::new(self, 24)
    }
}
#[doc = "Padding control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag028::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag028::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jtag028Spec;
impl crate::RegisterSpec for Jtag028Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtag028::R`](R) reader structure"]
impl crate::Readable for Jtag028Spec {}
#[doc = "`write(|w| ..)` method takes [`jtag028::W`](W) writer structure"]
impl crate::Writable for Jtag028Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JTAG028 to value 0"]
impl crate::Resettable for Jtag028Spec {}
