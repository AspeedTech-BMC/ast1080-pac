#[doc = "Register `I2CG0C` reader"]
pub type R = crate::R<I2cg0cSpec>;
#[doc = "Register `I2CG0C` writer"]
pub type W = crate::W<I2cg0cSpec>;
#[doc = "Field `CLKDIVSEL` reader - CLK_DIV_SEL"]
pub type ClkdivselR = crate::BitReader;
#[doc = "Field `CLKDIVSEL` writer - CLK_DIV_SEL"]
pub type ClkdivselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEWREGMODE` reader - NEW_REG_MODE"]
pub type NewregmodeR = crate::BitReader;
#[doc = "Field `IRQSEPARATE` reader - IRQ_SEPARATE"]
pub type IrqseparateR = crate::BitReader;
#[doc = "Field `IRQSEPARATE` writer - IRQ_SEPARATE"]
pub type IrqseparateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRXMODE` reader - SRX_MODE"]
pub type SrxmodeR = crate::BitReader;
#[doc = "Field `SRXMODE` writer - SRX_MODE"]
pub type SrxmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `MTX2RXDLY` reader - MTX2RX_DLY"]
pub type Mtx2rxdlyR = crate::FieldReader;
#[doc = "Field `MTX2RXDLY` writer - MTX2RX_DLY"]
pub type Mtx2rxdlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 1 - CLK_DIV_SEL"]
    #[inline(always)]
    pub fn clkdivsel(&self) -> ClkdivselR {
        ClkdivselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NEW_REG_MODE"]
    #[inline(always)]
    pub fn newregmode(&self) -> NewregmodeR {
        NewregmodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IRQ_SEPARATE"]
    #[inline(always)]
    pub fn irqseparate(&self) -> IrqseparateR {
        IrqseparateR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SRX_MODE"]
    #[inline(always)]
    pub fn srxmode(&self) -> SrxmodeR {
        SrxmodeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - MTX2RX_DLY"]
    #[inline(always)]
    pub fn mtx2rxdly(&self) -> Mtx2rxdlyR {
        Mtx2rxdlyR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - CLK_DIV_SEL"]
    #[inline(always)]
    pub fn clkdivsel(&mut self) -> ClkdivselW<I2cg0cSpec> {
        ClkdivselW::new(self, 1)
    }
    #[doc = "Bit 3 - IRQ_SEPARATE"]
    #[inline(always)]
    pub fn irqseparate(&mut self) -> IrqseparateW<I2cg0cSpec> {
        IrqseparateW::new(self, 3)
    }
    #[doc = "Bit 4 - SRX_MODE"]
    #[inline(always)]
    pub fn srxmode(&mut self) -> SrxmodeW<I2cg0cSpec> {
        SrxmodeW::new(self, 4)
    }
    #[doc = "Bits 8:11 - MTX2RX_DLY"]
    #[inline(always)]
    pub fn mtx2rxdly(&mut self) -> Mtx2rxdlyW<I2cg0cSpec> {
        Mtx2rxdlyW::new(self, 8)
    }
}
#[doc = "Global Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cg0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cg0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cg0cSpec;
impl crate::RegisterSpec for I2cg0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cg0c::R`](R) reader structure"]
impl crate::Readable for I2cg0cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cg0c::W`](W) writer structure"]
impl crate::Writable for I2cg0cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CG0C to value 0x04"]
impl crate::Resettable for I2cg0cSpec {
    const RESET_VALUE: u32 = 0x04;
}
