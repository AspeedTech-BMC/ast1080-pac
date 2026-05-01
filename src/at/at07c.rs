#[doc = "Register `AT07C` reader"]
pub type R = crate::R<At07cSpec>;
#[doc = "Register `AT07C` writer"]
pub type W = crate::W<At07cSpec>;
#[doc = "Field `ATTDDATA` reader - AT_TD_DATA"]
pub type AttddataR = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `ATTDDATAVLD` reader - AT_TD_DATA_VLD"]
pub type AttddatavldR = crate::BitReader;
#[doc = "Field `ATTDDATAVLD` writer - AT_TD_DATA_VLD"]
pub type AttddatavldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATTDERROR` reader - AT_TD_ERROR"]
pub type AttderrorR = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - AT_TD_DATA"]
    #[inline(always)]
    pub fn attddata(&self) -> AttddataR {
        AttddataR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - AT_TD_DATA_VLD"]
    #[inline(always)]
    pub fn attddatavld(&self) -> AttddatavldR {
        AttddatavldR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AT_TD_ERROR"]
    #[inline(always)]
    pub fn attderror(&self) -> AttderrorR {
        AttderrorR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - AT_TD_DATA_VLD"]
    #[inline(always)]
    pub fn attddatavld(&mut self) -> AttddatavldW<At07cSpec> {
        AttddatavldW::new(self, 16)
    }
}
#[doc = "TSENSE Status\n\nYou can [`read`](crate::Reg::read) this register and get [`at07c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at07c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At07cSpec;
impl crate::RegisterSpec for At07cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at07c::R`](R) reader structure"]
impl crate::Readable for At07cSpec {}
#[doc = "`write(|w| ..)` method takes [`at07c::W`](W) writer structure"]
impl crate::Writable for At07cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT07C to value 0"]
impl crate::Resettable for At07cSpec {}
