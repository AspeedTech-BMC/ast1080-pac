#[doc = "Register `AT028` reader"]
pub type R = crate::R<At028Spec>;
#[doc = "Register `AT028` writer"]
pub type W = crate::W<At028Spec>;
#[doc = "Field `ATCAMCHRINTEN` reader - AT_CAM_CHR_INT_EN"]
pub type AtcamchrintenR = crate::BitReader;
#[doc = "Field `ATCAMCHRINTEN` writer - AT_CAM_CHR_INT_EN"]
pub type AtcamchrintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCAMCHFINTEN` reader - AT_CAM_CHF_INT_EN"]
pub type AtcamchfintenR = crate::BitReader;
#[doc = "Field `ATCAMCHFINTEN` writer - AT_CAM_CHF_INT_EN"]
pub type AtcamchfintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCAMGDINTEN` reader - AT_CAM_GD_INT_EN"]
pub type AtcamgdintenR = crate::BitReader;
#[doc = "Field `ATCAMGDINTEN` writer - AT_CAM_GD_INT_EN"]
pub type AtcamgdintenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AT_CAM_CHR_INT_EN"]
    #[inline(always)]
    pub fn atcamchrinten(&self) -> AtcamchrintenR {
        AtcamchrintenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AT_CAM_CHF_INT_EN"]
    #[inline(always)]
    pub fn atcamchfinten(&self) -> AtcamchfintenR {
        AtcamchfintenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AT_CAM_GD_INT_EN"]
    #[inline(always)]
    pub fn atcamgdinten(&self) -> AtcamgdintenR {
        AtcamgdintenR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AT_CAM_CHR_INT_EN"]
    #[inline(always)]
    pub fn atcamchrinten(&mut self) -> AtcamchrintenW<At028Spec> {
        AtcamchrintenW::new(self, 0)
    }
    #[doc = "Bit 1 - AT_CAM_CHF_INT_EN"]
    #[inline(always)]
    pub fn atcamchfinten(&mut self) -> AtcamchfintenW<At028Spec> {
        AtcamchfintenW::new(self, 1)
    }
    #[doc = "Bit 2 - AT_CAM_GD_INT_EN"]
    #[inline(always)]
    pub fn atcamgdinten(&mut self) -> AtcamgdintenW<At028Spec> {
        AtcamgdintenW::new(self, 2)
    }
}
#[doc = "Clock Attack Monitor Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`at028::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at028::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At028Spec;
impl crate::RegisterSpec for At028Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at028::R`](R) reader structure"]
impl crate::Readable for At028Spec {}
#[doc = "`write(|w| ..)` method takes [`at028::W`](W) writer structure"]
impl crate::Writable for At028Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT028 to value 0"]
impl crate::Resettable for At028Spec {}
