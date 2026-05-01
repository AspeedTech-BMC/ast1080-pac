#[doc = "Register `AT014` reader"]
pub type R = crate::R<At014Spec>;
#[doc = "Register `AT014` writer"]
pub type W = crate::W<At014Spec>;
#[doc = "Field `ATCAMCHREN` reader - AT_CAM_CHR_EN"]
pub type AtcamchrenR = crate::BitReader;
#[doc = "Field `ATCAMCHREN` writer - AT_CAM_CHR_EN"]
pub type AtcamchrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCAMCHFEN` reader - AT_CAM_CHF_EN"]
pub type AtcamchfenR = crate::BitReader;
#[doc = "Field `ATCAMCHFEN` writer - AT_CAM_CHF_EN"]
pub type AtcamchfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCAMFQCEN` reader - AT_CAM_FQC_EN"]
pub type AtcamfqcenR = crate::BitReader;
#[doc = "Field `ATCAMFQCEN` writer - AT_CAM_FQC_EN"]
pub type AtcamfqcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCAMGDEN` reader - AT_CAM_GD_EN"]
pub type AtcamgdenR = crate::BitReader;
#[doc = "Field `ATCAMGDEN` writer - AT_CAM_GD_EN"]
pub type AtcamgdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCAMINTEN` reader - AT_CAM_INT_EN"]
pub type AtcamintenR = crate::BitReader;
#[doc = "Field `ATCAMINTEN` writer - AT_CAM_INT_EN"]
pub type AtcamintenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AT_CAM_CHR_EN"]
    #[inline(always)]
    pub fn atcamchren(&self) -> AtcamchrenR {
        AtcamchrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AT_CAM_CHF_EN"]
    #[inline(always)]
    pub fn atcamchfen(&self) -> AtcamchfenR {
        AtcamchfenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AT_CAM_FQC_EN"]
    #[inline(always)]
    pub fn atcamfqcen(&self) -> AtcamfqcenR {
        AtcamfqcenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AT_CAM_GD_EN"]
    #[inline(always)]
    pub fn atcamgden(&self) -> AtcamgdenR {
        AtcamgdenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AT_CAM_INT_EN"]
    #[inline(always)]
    pub fn atcaminten(&self) -> AtcamintenR {
        AtcamintenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AT_CAM_CHR_EN"]
    #[inline(always)]
    pub fn atcamchren(&mut self) -> AtcamchrenW<At014Spec> {
        AtcamchrenW::new(self, 0)
    }
    #[doc = "Bit 1 - AT_CAM_CHF_EN"]
    #[inline(always)]
    pub fn atcamchfen(&mut self) -> AtcamchfenW<At014Spec> {
        AtcamchfenW::new(self, 1)
    }
    #[doc = "Bit 2 - AT_CAM_FQC_EN"]
    #[inline(always)]
    pub fn atcamfqcen(&mut self) -> AtcamfqcenW<At014Spec> {
        AtcamfqcenW::new(self, 2)
    }
    #[doc = "Bit 3 - AT_CAM_GD_EN"]
    #[inline(always)]
    pub fn atcamgden(&mut self) -> AtcamgdenW<At014Spec> {
        AtcamgdenW::new(self, 3)
    }
    #[doc = "Bit 4 - AT_CAM_INT_EN"]
    #[inline(always)]
    pub fn atcaminten(&mut self) -> AtcamintenW<At014Spec> {
        AtcamintenW::new(self, 4)
    }
}
#[doc = "Clock Attack Monitor Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`at014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At014Spec;
impl crate::RegisterSpec for At014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at014::R`](R) reader structure"]
impl crate::Readable for At014Spec {}
#[doc = "`write(|w| ..)` method takes [`at014::W`](W) writer structure"]
impl crate::Writable for At014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT014 to value 0"]
impl crate::Resettable for At014Spec {}
