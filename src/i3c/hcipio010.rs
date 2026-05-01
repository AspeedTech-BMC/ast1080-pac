#[doc = "Register `HCIPIO010` reader"]
pub type R = crate::R<Hcipio010Spec>;
#[doc = "Register `HCIPIO010` writer"]
pub type W = crate::W<Hcipio010Spec>;
#[doc = "Field `REGCMDEMPTYBUFTHLD` reader - REG_CMD_EMPTY_BUF_THLD"]
pub type RegcmdemptybufthldR = crate::FieldReader;
#[doc = "Field `REGCMDEMPTYBUFTHLD` writer - REG_CMD_EMPTY_BUF_THLD"]
pub type RegcmdemptybufthldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGRESPBUFTHLD` reader - REG_RESP_BUF_THLD"]
pub type RegrespbufthldR = crate::FieldReader;
#[doc = "Field `REGRESPBUFTHLD` writer - REG_RESP_BUF_THLD"]
pub type RegrespbufthldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGIBIDATASEGMENTSIZE` reader - REG_IBI_DATA_SEGMENT_SIZE"]
pub type RegibidatasegmentsizeR = crate::FieldReader;
#[doc = "Field `REGIBIDATASEGMENTSIZE` writer - REG_IBI_DATA_SEGMENT_SIZE"]
pub type RegibidatasegmentsizeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGIBISTATUSTHLD` reader - REG_IBI_STATUS_THLD"]
pub type RegibistatusthldR = crate::FieldReader;
#[doc = "Field `REGIBISTATUSTHLD` writer - REG_IBI_STATUS_THLD"]
pub type RegibistatusthldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - REG_CMD_EMPTY_BUF_THLD"]
    #[inline(always)]
    pub fn regcmdemptybufthld(&self) -> RegcmdemptybufthldR {
        RegcmdemptybufthldR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_RESP_BUF_THLD"]
    #[inline(always)]
    pub fn regrespbufthld(&self) -> RegrespbufthldR {
        RegrespbufthldR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_IBI_DATA_SEGMENT_SIZE"]
    #[inline(always)]
    pub fn regibidatasegmentsize(&self) -> RegibidatasegmentsizeR {
        RegibidatasegmentsizeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - REG_IBI_STATUS_THLD"]
    #[inline(always)]
    pub fn regibistatusthld(&self) -> RegibistatusthldR {
        RegibistatusthldR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_CMD_EMPTY_BUF_THLD"]
    #[inline(always)]
    pub fn regcmdemptybufthld(&mut self) -> RegcmdemptybufthldW<Hcipio010Spec> {
        RegcmdemptybufthldW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_RESP_BUF_THLD"]
    #[inline(always)]
    pub fn regrespbufthld(&mut self) -> RegrespbufthldW<Hcipio010Spec> {
        RegrespbufthldW::new(self, 8)
    }
    #[doc = "Bits 16:23 - REG_IBI_DATA_SEGMENT_SIZE"]
    #[inline(always)]
    pub fn regibidatasegmentsize(&mut self) -> RegibidatasegmentsizeW<Hcipio010Spec> {
        RegibidatasegmentsizeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - REG_IBI_STATUS_THLD"]
    #[inline(always)]
    pub fn regibistatusthld(&mut self) -> RegibistatusthldW<Hcipio010Spec> {
        RegibistatusthldW::new(self, 24)
    }
}
#[doc = "QUEUE\\_THLD\\_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcipio010Spec;
impl crate::RegisterSpec for Hcipio010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcipio010::R`](R) reader structure"]
impl crate::Readable for Hcipio010Spec {}
#[doc = "`write(|w| ..)` method takes [`hcipio010::W`](W) writer structure"]
impl crate::Writable for Hcipio010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIPIO010 to value 0x0101_0101"]
impl crate::Resettable for Hcipio010Spec {
    const RESET_VALUE: u32 = 0x0101_0101;
}
