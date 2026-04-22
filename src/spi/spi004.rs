#[doc = "Register `SPI004` reader"]
pub type R = crate::R<Spi004Spec>;
#[doc = "Register `SPI004` writer"]
pub type W = crate::W<Spi004Spec>;
#[doc = "Field `CEC0A4BYTE` reader - CEC0_A4BYTE"]
pub type Cec0a4byteR = crate::BitReader;
#[doc = "Field `CEC0A4BYTE` writer - CEC0_A4BYTE"]
pub type Cec0a4byteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC1A4BYTE` reader - CEC1_A4BYTE"]
pub type Cec1a4byteR = crate::BitReader;
#[doc = "Field `CEC1A4BYTE` writer - CEC1_A4BYTE"]
pub type Cec1a4byteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC2A4BYTE` reader - CEC2_A4BYTE"]
pub type Cec2a4byteR = crate::BitReader;
#[doc = "Field `CEC2A4BYTE` writer - CEC2_A4BYTE"]
pub type Cec2a4byteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC3A4BYTE` reader - CEC3_A4BYTE"]
pub type Cec3a4byteR = crate::BitReader;
#[doc = "Field `CEC3A4BYTE` writer - CEC3_A4BYTE"]
pub type Cec3a4byteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC0A4BYTESEL` reader - CEC0_A4BYTE_SEL"]
pub type Cec0a4byteselR = crate::BitReader;
#[doc = "Field `CEC0A4BYTESEL` writer - CEC0_A4BYTE_SEL"]
pub type Cec0a4byteselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC1A4BYTESEL` reader - CEC1_A4BYTE_SEL"]
pub type Cec1a4byteselR = crate::BitReader;
#[doc = "Field `CEC1A4BYTESEL` writer - CEC1_A4BYTE_SEL"]
pub type Cec1a4byteselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC2A4BYTESEL` reader - CEC2_A4BYTE_SEL"]
pub type Cec2a4byteselR = crate::BitReader;
#[doc = "Field `CEC2A4BYTESEL` writer - CEC2_A4BYTE_SEL"]
pub type Cec2a4byteselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC3A4BYTESEL` reader - CEC3_A4BYTE_SEL"]
pub type Cec3a4byteselR = crate::BitReader;
#[doc = "Field `CEC3A4BYTESEL` writer - CEC3_A4BYTE_SEL"]
pub type Cec3a4byteselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC0TCSHIGH` reader - CEC0_TCSHIGH"]
pub type Cec0tcshighR = crate::FieldReader;
#[doc = "Field `CEC0TCSHIGH` writer - CEC0_TCSHIGH"]
pub type Cec0tcshighW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CEC1TCSHIGH` reader - CEC1_TCSHIGH"]
pub type Cec1tcshighR = crate::FieldReader;
#[doc = "Field `CEC1TCSHIGH` writer - CEC1_TCSHIGH"]
pub type Cec1tcshighW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CEC2TCSHIGH` reader - CEC2_TCSHIGH"]
pub type Cec2tcshighR = crate::FieldReader;
#[doc = "Field `CEC2TCSHIGH` writer - CEC2_TCSHIGH"]
pub type Cec2tcshighW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CEC3TCSHIGH` reader - CEC3_TCSHIGH"]
pub type Cec3tcshighR = crate::FieldReader;
#[doc = "Field `CEC3TCSHIGH` writer - CEC3_TCSHIGH"]
pub type Cec3tcshighW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CEC0TCSLOWCNT` reader - CEC0_TCS_LOWCNT"]
pub type Cec0tcslowcntR = crate::FieldReader;
#[doc = "Field `CEC0TCSLOWCNT` writer - CEC0_TCS_LOWCNT"]
pub type Cec0tcslowcntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CEC1TCSLOWCNT` reader - CEC1_TCS_LOWCNT"]
pub type Cec1tcslowcntR = crate::FieldReader;
#[doc = "Field `CEC1TCSLOWCNT` writer - CEC1_TCS_LOWCNT"]
pub type Cec1tcslowcntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CEC2TCSLOWCNT` reader - CEC2_TCS_LOWCNT"]
pub type Cec2tcslowcntR = crate::FieldReader;
#[doc = "Field `CEC2TCSLOWCNT` writer - CEC2_TCS_LOWCNT"]
pub type Cec2tcslowcntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CEC3TCSLOWCNT` reader - CEC3_TCS_LOWCNT"]
pub type Cec3tcslowcntR = crate::FieldReader;
#[doc = "Field `CEC3TCSLOWCNT` writer - CEC3_TCS_LOWCNT"]
pub type Cec3tcslowcntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - CEC0_A4BYTE"]
    #[inline(always)]
    pub fn cec0a4byte(&self) -> Cec0a4byteR {
        Cec0a4byteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CEC1_A4BYTE"]
    #[inline(always)]
    pub fn cec1a4byte(&self) -> Cec1a4byteR {
        Cec1a4byteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CEC2_A4BYTE"]
    #[inline(always)]
    pub fn cec2a4byte(&self) -> Cec2a4byteR {
        Cec2a4byteR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CEC3_A4BYTE"]
    #[inline(always)]
    pub fn cec3a4byte(&self) -> Cec3a4byteR {
        Cec3a4byteR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CEC0_A4BYTE_SEL"]
    #[inline(always)]
    pub fn cec0a4bytesel(&self) -> Cec0a4byteselR {
        Cec0a4byteselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CEC1_A4BYTE_SEL"]
    #[inline(always)]
    pub fn cec1a4bytesel(&self) -> Cec1a4byteselR {
        Cec1a4byteselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CEC2_A4BYTE_SEL"]
    #[inline(always)]
    pub fn cec2a4bytesel(&self) -> Cec2a4byteselR {
        Cec2a4byteselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CEC3_A4BYTE_SEL"]
    #[inline(always)]
    pub fn cec3a4bytesel(&self) -> Cec3a4byteselR {
        Cec3a4byteselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - CEC0_TCSHIGH"]
    #[inline(always)]
    pub fn cec0tcshigh(&self) -> Cec0tcshighR {
        Cec0tcshighR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - CEC1_TCSHIGH"]
    #[inline(always)]
    pub fn cec1tcshigh(&self) -> Cec1tcshighR {
        Cec1tcshighR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - CEC2_TCSHIGH"]
    #[inline(always)]
    pub fn cec2tcshigh(&self) -> Cec2tcshighR {
        Cec2tcshighR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - CEC3_TCSHIGH"]
    #[inline(always)]
    pub fn cec3tcshigh(&self) -> Cec3tcshighR {
        Cec3tcshighR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - CEC0_TCS_LOWCNT"]
    #[inline(always)]
    pub fn cec0tcslowcnt(&self) -> Cec0tcslowcntR {
        Cec0tcslowcntR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - CEC1_TCS_LOWCNT"]
    #[inline(always)]
    pub fn cec1tcslowcnt(&self) -> Cec1tcslowcntR {
        Cec1tcslowcntR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - CEC2_TCS_LOWCNT"]
    #[inline(always)]
    pub fn cec2tcslowcnt(&self) -> Cec2tcslowcntR {
        Cec2tcslowcntR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - CEC3_TCS_LOWCNT"]
    #[inline(always)]
    pub fn cec3tcslowcnt(&self) -> Cec3tcslowcntR {
        Cec3tcslowcntR::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CEC0_A4BYTE"]
    #[inline(always)]
    pub fn cec0a4byte(&mut self) -> Cec0a4byteW<Spi004Spec> {
        Cec0a4byteW::new(self, 0)
    }
    #[doc = "Bit 1 - CEC1_A4BYTE"]
    #[inline(always)]
    pub fn cec1a4byte(&mut self) -> Cec1a4byteW<Spi004Spec> {
        Cec1a4byteW::new(self, 1)
    }
    #[doc = "Bit 2 - CEC2_A4BYTE"]
    #[inline(always)]
    pub fn cec2a4byte(&mut self) -> Cec2a4byteW<Spi004Spec> {
        Cec2a4byteW::new(self, 2)
    }
    #[doc = "Bit 3 - CEC3_A4BYTE"]
    #[inline(always)]
    pub fn cec3a4byte(&mut self) -> Cec3a4byteW<Spi004Spec> {
        Cec3a4byteW::new(self, 3)
    }
    #[doc = "Bit 4 - CEC0_A4BYTE_SEL"]
    #[inline(always)]
    pub fn cec0a4bytesel(&mut self) -> Cec0a4byteselW<Spi004Spec> {
        Cec0a4byteselW::new(self, 4)
    }
    #[doc = "Bit 5 - CEC1_A4BYTE_SEL"]
    #[inline(always)]
    pub fn cec1a4bytesel(&mut self) -> Cec1a4byteselW<Spi004Spec> {
        Cec1a4byteselW::new(self, 5)
    }
    #[doc = "Bit 6 - CEC2_A4BYTE_SEL"]
    #[inline(always)]
    pub fn cec2a4bytesel(&mut self) -> Cec2a4byteselW<Spi004Spec> {
        Cec2a4byteselW::new(self, 6)
    }
    #[doc = "Bit 7 - CEC3_A4BYTE_SEL"]
    #[inline(always)]
    pub fn cec3a4bytesel(&mut self) -> Cec3a4byteselW<Spi004Spec> {
        Cec3a4byteselW::new(self, 7)
    }
    #[doc = "Bits 8:9 - CEC0_TCSHIGH"]
    #[inline(always)]
    pub fn cec0tcshigh(&mut self) -> Cec0tcshighW<Spi004Spec> {
        Cec0tcshighW::new(self, 8)
    }
    #[doc = "Bits 10:11 - CEC1_TCSHIGH"]
    #[inline(always)]
    pub fn cec1tcshigh(&mut self) -> Cec1tcshighW<Spi004Spec> {
        Cec1tcshighW::new(self, 10)
    }
    #[doc = "Bits 12:13 - CEC2_TCSHIGH"]
    #[inline(always)]
    pub fn cec2tcshigh(&mut self) -> Cec2tcshighW<Spi004Spec> {
        Cec2tcshighW::new(self, 12)
    }
    #[doc = "Bits 14:15 - CEC3_TCSHIGH"]
    #[inline(always)]
    pub fn cec3tcshigh(&mut self) -> Cec3tcshighW<Spi004Spec> {
        Cec3tcshighW::new(self, 14)
    }
    #[doc = "Bits 16:17 - CEC0_TCS_LOWCNT"]
    #[inline(always)]
    pub fn cec0tcslowcnt(&mut self) -> Cec0tcslowcntW<Spi004Spec> {
        Cec0tcslowcntW::new(self, 16)
    }
    #[doc = "Bits 18:19 - CEC1_TCS_LOWCNT"]
    #[inline(always)]
    pub fn cec1tcslowcnt(&mut self) -> Cec1tcslowcntW<Spi004Spec> {
        Cec1tcslowcntW::new(self, 18)
    }
    #[doc = "Bits 20:21 - CEC2_TCS_LOWCNT"]
    #[inline(always)]
    pub fn cec2tcslowcnt(&mut self) -> Cec2tcslowcntW<Spi004Spec> {
        Cec2tcslowcntW::new(self, 20)
    }
    #[doc = "Bits 22:23 - CEC3_TCS_LOWCNT"]
    #[inline(always)]
    pub fn cec3tcslowcnt(&mut self) -> Cec3tcslowcntW<Spi004Spec> {
        Cec3tcslowcntW::new(self, 22)
    }
}
#[doc = "CE Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi004Spec;
impl crate::RegisterSpec for Spi004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi004::R`](R) reader structure"]
impl crate::Readable for Spi004Spec {}
#[doc = "`write(|w| ..)` method takes [`spi004::W`](W) writer structure"]
impl crate::Writable for Spi004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI004 to value 0xaa00"]
impl crate::Resettable for Spi004Spec {
    const RESET_VALUE: u32 = 0xaa00;
}
