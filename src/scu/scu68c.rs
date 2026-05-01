#[doc = "Register `SCU68C` reader"]
pub type R = crate::R<Scu68cSpec>;
#[doc = "Register `SCU68C` writer"]
pub type W = crate::W<Scu68cSpec>;
#[doc = "Field `SCUIOCTRLDQ8` reader - SCU_IO_CTRL_DQ8"]
pub type Scuioctrldq8R = crate::FieldReader;
#[doc = "Field `SCUIOCTRLDQ8` writer - SCU_IO_CTRL_DQ8"]
pub type Scuioctrldq8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLDQ9` reader - SCU_IO_CTRL_DQ9"]
pub type Scuioctrldq9R = crate::FieldReader;
#[doc = "Field `SCUIOCTRLDQ9` writer - SCU_IO_CTRL_DQ9"]
pub type Scuioctrldq9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLDQ10` reader - SCU_IO_CTRL_DQ10"]
pub type Scuioctrldq10R = crate::FieldReader;
#[doc = "Field `SCUIOCTRLDQ10` writer - SCU_IO_CTRL_DQ10"]
pub type Scuioctrldq10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLDQ11` reader - SCU_IO_CTRL_DQ11"]
pub type Scuioctrldq11R = crate::FieldReader;
#[doc = "Field `SCUIOCTRLDQ11` writer - SCU_IO_CTRL_DQ11"]
pub type Scuioctrldq11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLDQ12` reader - SCU_IO_CTRL_DQ12"]
pub type Scuioctrldq12R = crate::FieldReader;
#[doc = "Field `SCUIOCTRLDQ12` writer - SCU_IO_CTRL_DQ12"]
pub type Scuioctrldq12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLDQ13` reader - SCU_IO_CTRL_DQ13"]
pub type Scuioctrldq13R = crate::FieldReader;
#[doc = "Field `SCUIOCTRLDQ13` writer - SCU_IO_CTRL_DQ13"]
pub type Scuioctrldq13W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLDQ14` reader - SCU_IO_CTRL_DQ14"]
pub type Scuioctrldq14R = crate::FieldReader;
#[doc = "Field `SCUIOCTRLDQ14` writer - SCU_IO_CTRL_DQ14"]
pub type Scuioctrldq14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLDQ15` reader - SCU_IO_CTRL_DQ15"]
pub type Scuioctrldq15R = crate::FieldReader;
#[doc = "Field `SCUIOCTRLDQ15` writer - SCU_IO_CTRL_DQ15"]
pub type Scuioctrldq15W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - SCU_IO_CTRL_DQ8"]
    #[inline(always)]
    pub fn scuioctrldq8(&self) -> Scuioctrldq8R {
        Scuioctrldq8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SCU_IO_CTRL_DQ9"]
    #[inline(always)]
    pub fn scuioctrldq9(&self) -> Scuioctrldq9R {
        Scuioctrldq9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SCU_IO_CTRL_DQ10"]
    #[inline(always)]
    pub fn scuioctrldq10(&self) -> Scuioctrldq10R {
        Scuioctrldq10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SCU_IO_CTRL_DQ11"]
    #[inline(always)]
    pub fn scuioctrldq11(&self) -> Scuioctrldq11R {
        Scuioctrldq11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SCU_IO_CTRL_DQ12"]
    #[inline(always)]
    pub fn scuioctrldq12(&self) -> Scuioctrldq12R {
        Scuioctrldq12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SCU_IO_CTRL_DQ13"]
    #[inline(always)]
    pub fn scuioctrldq13(&self) -> Scuioctrldq13R {
        Scuioctrldq13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - SCU_IO_CTRL_DQ14"]
    #[inline(always)]
    pub fn scuioctrldq14(&self) -> Scuioctrldq14R {
        Scuioctrldq14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - SCU_IO_CTRL_DQ15"]
    #[inline(always)]
    pub fn scuioctrldq15(&self) -> Scuioctrldq15R {
        Scuioctrldq15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SCU_IO_CTRL_DQ8"]
    #[inline(always)]
    pub fn scuioctrldq8(&mut self) -> Scuioctrldq8W<Scu68cSpec> {
        Scuioctrldq8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - SCU_IO_CTRL_DQ9"]
    #[inline(always)]
    pub fn scuioctrldq9(&mut self) -> Scuioctrldq9W<Scu68cSpec> {
        Scuioctrldq9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - SCU_IO_CTRL_DQ10"]
    #[inline(always)]
    pub fn scuioctrldq10(&mut self) -> Scuioctrldq10W<Scu68cSpec> {
        Scuioctrldq10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - SCU_IO_CTRL_DQ11"]
    #[inline(always)]
    pub fn scuioctrldq11(&mut self) -> Scuioctrldq11W<Scu68cSpec> {
        Scuioctrldq11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - SCU_IO_CTRL_DQ12"]
    #[inline(always)]
    pub fn scuioctrldq12(&mut self) -> Scuioctrldq12W<Scu68cSpec> {
        Scuioctrldq12W::new(self, 16)
    }
    #[doc = "Bits 20:23 - SCU_IO_CTRL_DQ13"]
    #[inline(always)]
    pub fn scuioctrldq13(&mut self) -> Scuioctrldq13W<Scu68cSpec> {
        Scuioctrldq13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - SCU_IO_CTRL_DQ14"]
    #[inline(always)]
    pub fn scuioctrldq14(&mut self) -> Scuioctrldq14W<Scu68cSpec> {
        Scuioctrldq14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - SCU_IO_CTRL_DQ15"]
    #[inline(always)]
    pub fn scuioctrldq15(&mut self) -> Scuioctrldq15W<Scu68cSpec> {
        Scuioctrldq15W::new(self, 28)
    }
}
#[doc = "HRAM IO Control 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu68c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu68c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu68cSpec;
impl crate::RegisterSpec for Scu68cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu68c::R`](R) reader structure"]
impl crate::Readable for Scu68cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu68c::W`](W) writer structure"]
impl crate::Writable for Scu68cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU68C to value 0x1111_1111"]
impl crate::Resettable for Scu68cSpec {
    const RESET_VALUE: u32 = 0x1111_1111;
}
