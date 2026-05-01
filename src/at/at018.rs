#[doc = "Register `AT018` reader"]
pub type R = crate::R<At018Spec>;
#[doc = "Register `AT018` writer"]
pub type W = crate::W<At018Spec>;
#[doc = "Field `ATCAMCHRENCLR` reader - AT_CAM_CHR_EN_CLR"]
pub type AtcamchrenclrR = crate::BitReader;
#[doc = "Field `ATCAMCHRENCLR` writer - AT_CAM_CHR_EN_CLR"]
pub type AtcamchrenclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCAMCHFENCLR` reader - AT_CAM_CHF_EN_CLR"]
pub type AtcamchfenclrR = crate::BitReader;
#[doc = "Field `ATCAMCHFENCLR` writer - AT_CAM_CHF_EN_CLR"]
pub type AtcamchfenclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCAMFQCENCLR` reader - AT_CAM_FQC_EN_CLR"]
pub type AtcamfqcenclrR = crate::BitReader;
#[doc = "Field `ATCAMFQCENCLR` writer - AT_CAM_FQC_EN_CLR"]
pub type AtcamfqcenclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCAMGDENCLR` reader - AT_CAM_GD_EN_CLR"]
pub type AtcamgdenclrR = crate::BitReader;
#[doc = "Field `ATCAMGDENCLR` writer - AT_CAM_GD_EN_CLR"]
pub type AtcamgdenclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCAMINTENCLR` reader - AT_CAM_INT_EN_CLR"]
pub type AtcamintenclrR = crate::BitReader;
#[doc = "Field `ATCAMINTENCLR` writer - AT_CAM_INT_EN_CLR"]
pub type AtcamintenclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AT_CAM_CHR_EN_CLR"]
    #[inline(always)]
    pub fn atcamchrenclr(&self) -> AtcamchrenclrR {
        AtcamchrenclrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AT_CAM_CHF_EN_CLR"]
    #[inline(always)]
    pub fn atcamchfenclr(&self) -> AtcamchfenclrR {
        AtcamchfenclrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AT_CAM_FQC_EN_CLR"]
    #[inline(always)]
    pub fn atcamfqcenclr(&self) -> AtcamfqcenclrR {
        AtcamfqcenclrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AT_CAM_GD_EN_CLR"]
    #[inline(always)]
    pub fn atcamgdenclr(&self) -> AtcamgdenclrR {
        AtcamgdenclrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AT_CAM_INT_EN_CLR"]
    #[inline(always)]
    pub fn atcamintenclr(&self) -> AtcamintenclrR {
        AtcamintenclrR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AT_CAM_CHR_EN_CLR"]
    #[inline(always)]
    pub fn atcamchrenclr(&mut self) -> AtcamchrenclrW<At018Spec> {
        AtcamchrenclrW::new(self, 0)
    }
    #[doc = "Bit 1 - AT_CAM_CHF_EN_CLR"]
    #[inline(always)]
    pub fn atcamchfenclr(&mut self) -> AtcamchfenclrW<At018Spec> {
        AtcamchfenclrW::new(self, 1)
    }
    #[doc = "Bit 2 - AT_CAM_FQC_EN_CLR"]
    #[inline(always)]
    pub fn atcamfqcenclr(&mut self) -> AtcamfqcenclrW<At018Spec> {
        AtcamfqcenclrW::new(self, 2)
    }
    #[doc = "Bit 3 - AT_CAM_GD_EN_CLR"]
    #[inline(always)]
    pub fn atcamgdenclr(&mut self) -> AtcamgdenclrW<At018Spec> {
        AtcamgdenclrW::new(self, 3)
    }
    #[doc = "Bit 4 - AT_CAM_INT_EN_CLR"]
    #[inline(always)]
    pub fn atcamintenclr(&mut self) -> AtcamintenclrW<At018Spec> {
        AtcamintenclrW::new(self, 4)
    }
}
#[doc = "Clock Attack Monitor Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`at018::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at018::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At018Spec;
impl crate::RegisterSpec for At018Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at018::R`](R) reader structure"]
impl crate::Readable for At018Spec {}
#[doc = "`write(|w| ..)` method takes [`at018::W`](W) writer structure"]
impl crate::Writable for At018Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT018 to value 0"]
impl crate::Resettable for At018Spec {}
