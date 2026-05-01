#[doc = "Register `AT050` reader"]
pub type R = crate::R<At050Spec>;
#[doc = "Register `AT050` writer"]
pub type W = crate::W<At050Spec>;
#[doc = "Field `ATIREN` reader - AT_IR_EN"]
pub type AtirenR = crate::BitReader;
#[doc = "Field `ATIREN` writer - AT_IR_EN"]
pub type AtirenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATIRENHYS` reader - AT_IR_EN_HYS"]
pub type AtirenhysR = crate::BitReader;
#[doc = "Field `ATIRENHYS` writer - AT_IR_EN_HYS"]
pub type AtirenhysW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATIRTHRES` reader - AT_IR_THRES"]
pub type AtirthresR = crate::FieldReader;
#[doc = "Field `ATIRTHRES` writer - AT_IR_THRES"]
pub type AtirthresW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ATIRSELMON` reader - AT_IR_SEL_MON"]
pub type AtirselmonR = crate::FieldReader;
#[doc = "Field `ATIRSELMON` writer - AT_IR_SEL_MON"]
pub type AtirselmonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - AT_IR_EN"]
    #[inline(always)]
    pub fn atiren(&self) -> AtirenR {
        AtirenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AT_IR_EN_HYS"]
    #[inline(always)]
    pub fn atirenhys(&self) -> AtirenhysR {
        AtirenhysR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - AT_IR_THRES"]
    #[inline(always)]
    pub fn atirthres(&self) -> AtirthresR {
        AtirthresR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 30:31 - AT_IR_SEL_MON"]
    #[inline(always)]
    pub fn atirselmon(&self) -> AtirselmonR {
        AtirselmonR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AT_IR_EN"]
    #[inline(always)]
    pub fn atiren(&mut self) -> AtirenW<At050Spec> {
        AtirenW::new(self, 0)
    }
    #[doc = "Bit 1 - AT_IR_EN_HYS"]
    #[inline(always)]
    pub fn atirenhys(&mut self) -> AtirenhysW<At050Spec> {
        AtirenhysW::new(self, 1)
    }
    #[doc = "Bits 2:4 - AT_IR_THRES"]
    #[inline(always)]
    pub fn atirthres(&mut self) -> AtirthresW<At050Spec> {
        AtirthresW::new(self, 2)
    }
    #[doc = "Bits 30:31 - AT_IR_SEL_MON"]
    #[inline(always)]
    pub fn atirselmon(&mut self) -> AtirselmonW<At050Spec> {
        AtirselmonW::new(self, 30)
    }
}
#[doc = "IR Drop Control\n\nYou can [`read`](crate::Reg::read) this register and get [`at050::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at050::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At050Spec;
impl crate::RegisterSpec for At050Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at050::R`](R) reader structure"]
impl crate::Readable for At050Spec {}
#[doc = "`write(|w| ..)` method takes [`at050::W`](W) writer structure"]
impl crate::Writable for At050Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT050 to value 0"]
impl crate::Resettable for At050Spec {}
