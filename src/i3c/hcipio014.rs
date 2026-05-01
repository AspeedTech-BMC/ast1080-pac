#[doc = "Register `HCIPIO014` reader"]
pub type R = crate::R<Hcipio014Spec>;
#[doc = "Register `HCIPIO014` writer"]
pub type W = crate::W<Hcipio014Spec>;
#[doc = "Field `REGTXBUFTHLD` reader - REG_TX_BUF_THLD"]
pub type RegtxbufthldR = crate::FieldReader;
#[doc = "Field `REGTXBUFTHLD` writer - REG_TX_BUF_THLD"]
pub type RegtxbufthldW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `REGRXBUFTHLD` reader - REG_RX_BUF_THLD"]
pub type RegrxbufthldR = crate::FieldReader;
#[doc = "Field `REGRXBUFTHLD` writer - REG_RX_BUF_THLD"]
pub type RegrxbufthldW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `REGTXSTARTTHLD` reader - REG_TX_START_THLD"]
pub type RegtxstartthldR = crate::FieldReader;
#[doc = "Field `REGTXSTARTTHLD` writer - REG_TX_START_THLD"]
pub type RegtxstartthldW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGRXSTARTTHLD` reader - REG_RX_START_THLD"]
pub type RegrxstartthldR = crate::FieldReader;
#[doc = "Field `REGRXSTARTTHLD` writer - REG_RX_START_THLD"]
pub type RegrxstartthldW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - REG_TX_BUF_THLD"]
    #[inline(always)]
    pub fn regtxbufthld(&self) -> RegtxbufthldR {
        RegtxbufthldR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - REG_RX_BUF_THLD"]
    #[inline(always)]
    pub fn regrxbufthld(&self) -> RegrxbufthldR {
        RegrxbufthldR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - REG_TX_START_THLD"]
    #[inline(always)]
    pub fn regtxstartthld(&self) -> RegtxstartthldR {
        RegtxstartthldR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:23 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - REG_RX_START_THLD"]
    #[inline(always)]
    pub fn regrxstartthld(&self) -> RegrxstartthldR {
        RegrxstartthldR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_TX_BUF_THLD"]
    #[inline(always)]
    pub fn regtxbufthld(&mut self) -> RegtxbufthldW<Hcipio014Spec> {
        RegtxbufthldW::new(self, 0)
    }
    #[doc = "Bits 8:10 - REG_RX_BUF_THLD"]
    #[inline(always)]
    pub fn regrxbufthld(&mut self) -> RegrxbufthldW<Hcipio014Spec> {
        RegrxbufthldW::new(self, 8)
    }
    #[doc = "Bits 16:18 - REG_TX_START_THLD"]
    #[inline(always)]
    pub fn regtxstartthld(&mut self) -> RegtxstartthldW<Hcipio014Spec> {
        RegtxstartthldW::new(self, 16)
    }
    #[doc = "Bits 24:26 - REG_RX_START_THLD"]
    #[inline(always)]
    pub fn regrxstartthld(&mut self) -> RegrxstartthldW<Hcipio014Spec> {
        RegrxstartthldW::new(self, 24)
    }
}
#[doc = "DATA\\_BUFFER\\_THLD\\_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcipio014Spec;
impl crate::RegisterSpec for Hcipio014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcipio014::R`](R) reader structure"]
impl crate::Readable for Hcipio014Spec {}
#[doc = "`write(|w| ..)` method takes [`hcipio014::W`](W) writer structure"]
impl crate::Writable for Hcipio014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIPIO014 to value 0x0101_0404"]
impl crate::Resettable for Hcipio014Spec {
    const RESET_VALUE: u32 = 0x0101_0404;
}
